// Generated macro for impl_2802 (impl)
macro_rules! Depcrate_traversalsimpl_2802 {
() => {
// Module: crate::traversals
// Provides: {"impl_2802"}
// Dependencies: {}
impl Dfs { # [doc = " Construct a new depth-first traversal."] pub fn new () -> Self { Self :: default () } # [doc = " Perform a depth-first traversal over the given function."] # [doc = ""] # [doc = " Yields pairs of `(Event, ir::Block)`."] # [doc = ""] # [doc = " This iterator can be used to perform either pre- or post-order"] # [doc = " traversals, or a combination of the two."] pub fn iter < 'a > (& 'a mut self , func : & 'a ir :: Function) -> DfsIter < 'a > { self . clear () ; if let Some (e) = func . layout . entry_block () { self . stack . push ((Event :: Enter , e)) ; } DfsIter { dfs : self , func } } # [doc = " Perform a pre-order traversal over the given function."] # [doc = ""] # [doc = " Yields `ir::Block` items."] pub fn pre_order_iter < 'a > (& 'a mut self , func : & 'a ir :: Function) -> DfsPreOrderIter < 'a > { DfsPreOrderIter (self . iter (func)) } # [doc = " Perform a post-order traversal over the given function."] # [doc = ""] # [doc = " Yields `ir::Block` items."] pub fn post_order_iter < 'a > (& 'a mut self , func : & 'a ir :: Function) -> DfsPostOrderIter < 'a > { DfsPostOrderIter (self . iter (func)) } # [doc = " Clear this DFS, but keep its allocations for future reuse."] pub fn clear (& mut self) { let Dfs { stack , seen } = self ; stack . clear () ; seen . clear () ; } }
};
}
