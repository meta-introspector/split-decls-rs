use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `imara-diff` supports multiple different algorithms
/// for computing an edit sequence.
/// These algorithms have different performance and all produce different output.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Algorithm {
    /// A variation of the [`patience` diff algorithm described by Bram Cohen's blog post](https://bramcohen.livejournal.com/73318.html)
    /// that uses a histogram to find the least common LCS.
    /// Just like the `patience` diff algorithm, this algorithm usually produces
    /// more human readable output then myers algorithm.
    /// However compared to the `patience` diff algorithm (which is slower then myers algorithm),
    /// the Histogram algorithm performs much better.
    ///
    /// The implementation here was originally ported from `git` but has been significantly
    /// modified to improve performance.
    /// As a result it consistently **performs better then myers algorithm** (5%-100%) over
    /// a wide variety of test data. For example a benchmark of diffing linux kernel commits is shown below:
    #[cfg_attr(
        doc,
        doc = concat!(
            "<img width=\"600\" class=\"figure\" src=\"data:image/svg+xml;base64,",
            include_str!("../plots/linux_speedup.svg.base64"),
            "\"></img>"
        )
    )]
    ///
    /// For pathological subsequences that only contain highly repeating tokens (64+ occurrences)
    /// the algorithm falls back on Myers algorithm (with heuristics) to avoid quadratic behavior.
    ///
    /// Compared to Myers algorithm, the Histogram diff algorithm is more focused on providing
    /// human readable diffs instead of minimal diffs. In practice this means that the edit-sequences
    /// produced by the histogram diff are often longer then those produced by Myers algorithm.
    ///
    /// The heuristic used by the histogram diff does not work well for inputs with small (often repeated)
    /// tokens. For example **character diffs do not work well** as most (english) text is made up of
    /// a fairly small set of characters. The `Histogram` algorithm will automatically these cases and
    /// fallback to Myers algorithm. However this detection has a nontrivial overhead, so
    /// if its known upfront that the sort of tokens is very small `Myers` algorithm should
    /// be used instead.
    #[default]
    Histogram,
    /// An implementation of the linear space variant of
    /// [Myers  `O((N+M)D)` algorithm](http://www.xmailserver.org/diff2.pdf).
    /// The algorithm is enhanced with preprocessing that removes
    /// tokens that don't occur in the other file at all.
    /// Furthermore two heuristics to the middle snake search are implemented
    /// that ensure reasonable runtime (mostly linear time complexity) even for large files.
    ///
    /// Due to the divide and conquer nature of the algorithm
    /// the edit sequenced produced are still fairly small even when the middle snake
    /// search is aborted by a heuristic.
    /// However, the produced edit sequences are not guaranteed to be fully minimal.
    /// If that property is vital to you, use the `MyersMinimal` algorithm instead.
    ///
    /// The implementation (including the preprocessing) are mostly
    /// ported from `git` and `gnu-diff` where Myers algorithm is used
    /// as the default diff algorithm.
    /// Therefore the used heuristics have been heavily battle tested and
    /// are known to behave well over a large variety of inputs
    Myers,
    /// Same as `Myers` but the early abort heuristics are disabled to guarantee
    /// a minimal edit sequence.
    /// This can mean significant slowdown in pathological cases.
    MyersMinimal,
}
