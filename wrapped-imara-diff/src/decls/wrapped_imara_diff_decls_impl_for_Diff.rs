use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Diff {
    /// Computes an edit-script that transforms `input.before` into `input.after` using
    /// the specified `algorithm`
    pub fn compute<T>(algorithm: Algorithm, input: &InternedInput<T>) -> Diff {
        let mut diff = Diff::default();
        diff.compute_with(
            algorithm,
            &input.before,
            &input.after,
            input.interner.num_tokens(),
        );
        diff
    }
    /// Computes an edit-script that transforms `before` into `after` using
    /// the specified `algorithm`.
    pub fn compute_with(
        &mut self,
        algorithm: Algorithm,
        mut before: &[Token],
        mut after: &[Token],
        num_tokens: u32,
    ) {
        assert!(
            before.len() < i32::MAX as usize,
            "imara-diff only supports up to {} tokens",
            i32::MAX
        );
        assert!(
            after.len() < i32::MAX as usize,
            "imara-diff only supports up to {} tokens",
            i32::MAX
        );
        self.removed.clear();
        self.added.clear();
        self.removed.resize(before.len(), false);
        self.added.resize(after.len(), false);
        let common_prefix = strip_common_prefix(&mut before, &mut after) as usize;
        let common_postfix = strip_common_postfix(&mut before, &mut after);
        let range = common_prefix..self.removed.len() - common_postfix as usize;
        let removed = &mut self.removed[range];
        let range = common_prefix..self.added.len() - common_postfix as usize;
        let added = &mut self.added[range];
        match algorithm {
            Algorithm::Histogram => histogram::diff(before, after, removed, added, num_tokens),
            Algorithm::Myers => myers::diff(before, after, removed, added, false),
            Algorithm::MyersMinimal => myers::diff(before, after, removed, added, true),
        }
    }
    pub fn count_additions(&self) -> u32 {
        self.added.iter().map(|&added| added as u32).sum()
    }
    pub fn count_removals(&self) -> u32 {
        self.removed.iter().map(|&removed| removed as u32).sum()
    }
    pub fn is_removed(&self, token_idx: u32) -> bool {
        self.removed[token_idx as usize]
    }
    pub fn is_added(&self, token_idx: u32) -> bool {
        self.added[token_idx as usize]
    }
    /// Postprocesses the diff to make it more human readable. Certain bvhunks
    /// have an ambiguous placement (even in a minimal diff) where they can move
    /// downward or upward by removing a token (line) at the start and adding
    /// one at the end (or the other way around). The postprocessing adjust
    /// these hunks according to a couple rules:
    ///
    /// * Always merge multiple hunks if possible.
    /// * Always try to create a single MODIFY hunk instead of multiple disjoint
    ///   ADDED/REMOVED hunks
    /// * Move sliders as far down as possible.
    pub fn postprocess_no_heuristic<T>(&mut self, input: &InternedInput<T>) {
        self.postprocess_with_heuristic(input, NoSliderHeuristic)
    }
    /// Postprocesses the diff to make it more human readable. Certain bvhunks
    /// have an ambiguous placement (even in a minimal diff) where they can move
    /// downward or upward by removing a token (line) at the start and adding
    /// one at the end (or the other way around). The postprocessing adjust
    /// these hunks according to a couple rules:
    ///
    /// * Always merge multiple hunks if possible.
    /// * Always try to create a single MODIFY hunk instead of multiple disjoint
    ///   ADDED/REMOVED hunks
    /// * based on a lines indentation level heuristically compute the most
    ///   intuitive location to split lines.
    /// * Move sliders as far down as possible.
    pub fn postprocess_lines<T: AsRef<[u8]>>(&mut self, input: &InternedInput<T>) {
        self.postprocess_with_heuristic(
            input,
            IndentHeuristic::new(|token| {
                IndentLevel::for_ascii_line(input.interner[token].as_ref().iter().copied(), 8)
            }),
        )
    }
    /// An iterator that yields the changed hunks in this diff
    pub fn hunks(&self) -> HunkIter<'_> {
        HunkIter {
            removed: self.removed.iter(),
            added: self.added.iter(),
            pos_before: 0,
            pos_after: 0,
        }
    }
}
