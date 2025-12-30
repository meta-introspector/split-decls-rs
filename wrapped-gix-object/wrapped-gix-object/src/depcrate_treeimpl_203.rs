// Generated macro for impl_203 (impl)
macro_rules! Depcrate_treeimpl_203 {
() => {
// Module: crate::tree
// Provides: {"impl_203"}
// Dependencies: {}
impl Ord for EntryRef < '_ > { fn cmp (& self , b : & Self) -> Ordering { let a = self ; let common = a . filename . len () . min (b . filename . len ()) ; a . filename [.. common] . cmp (& b . filename [.. common]) . then_with (| | { let a = a . filename . get (common) . or_else (| | a . mode . is_tree () . then_some (& b'/')) ; let b = b . filename . get (common) . or_else (| | b . mode . is_tree () . then_some (& b'/')) ; a . cmp (& b) }) } }
};
}
