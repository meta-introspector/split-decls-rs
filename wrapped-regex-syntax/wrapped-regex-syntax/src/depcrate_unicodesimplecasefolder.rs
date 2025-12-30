// Generated macro for SimpleCaseFolder (struct)
macro_rules! Depcrate_unicodeSimpleCaseFolder {
() => {
// Module: crate::unicode
// Provides: {"SimpleCaseFolder"}
// Dependencies: {}
# [doc = " A state oriented traverser of the simple case folding table."] # [doc = ""] # [doc = " A case folder can be constructed via `SimpleCaseFolder::new()`, which will"] # [doc = " return an error if the underlying case folding table is unavailable."] # [doc = ""] # [doc = " After construction, it is expected that callers will use"] # [doc = " `SimpleCaseFolder::mapping` by calling it with codepoints in strictly"] # [doc = " increasing order. For example, calling it on `b` and then on `a` is illegal"] # [doc = " and will result in a panic."] # [doc = ""] # [doc = " The main idea of this type is that it tries hard to make mapping lookups"] # [doc = " fast by exploiting the structure of the underlying table, and the ordering"] # [doc = " assumption enables this."] # [derive (Debug)] pub struct SimpleCaseFolder { # [doc = " The simple case fold table. It's a sorted association list, where the"] # [doc = " keys are Unicode scalar values and the values are the corresponding"] # [doc = " equivalence class (not including the key) of the \"simple\" case folded"] # [doc = " Unicode scalar values."] table : & 'static [(char , & 'static [char])] , # [doc = " The last codepoint that was used for a lookup."] last : Option < char > , # [doc = " The index to the entry in `table` corresponding to the smallest key `k`"] # [doc = " such that `k > k0`, where `k0` is the most recent key lookup. Note that"] # [doc = " in particular, `k0` may not be in the table!"] next : usize , }
};
}
