// Generated macro for impl_170 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrowsimpl_170 {
() => {
// Module: crate::borrow_tracker::stacked_borrows
// Provides: {"impl_170"}
// Dependencies: {}
# [doc = " Map per-stack operations to higher-level per-location-range operations."] impl < 'tcx > Stacks { # [doc = " Creates a new stack with an initial tag. For diagnostic purposes, we also need to know"] # [doc = " the [`AllocId`] of the allocation this is associated with."] fn new (size : Size , perm : Permission , tag : BorTag , id : AllocId , machine : & MiriMachine < '_ > ,) -> Self { let item = Item :: new (tag , perm , false) ; let stack = Stack :: new (item) ; Stacks { stacks : DedupRangeMap :: new (size , stack) , history : AllocHistory :: new (id , item , machine) , exposed_tags : FxHashSet :: default () , } } # [doc = " Call `f` on every stack in the range."] fn for_each (& mut self , range : AllocRange , mut dcx_builder : DiagnosticCxBuilder < '_ , 'tcx > , mut f : impl FnMut (& mut Stack , & mut DiagnosticCx < '_ , '_ , 'tcx > , & mut FxHashSet < BorTag > ,) -> InterpResult < 'tcx > ,) -> InterpResult < 'tcx > { for (stack_range , stack) in self . stacks . iter_mut (range . start , range . size) { let mut dcx = dcx_builder . build (& mut self . history , Size :: from_bytes (stack_range . start)) ; f (stack , & mut dcx , & mut self . exposed_tags) ? ; dcx_builder = dcx . unbuild () ; } interp_ok (()) } }
};
}
