// Generated macro for CatchUnwindData (struct)
macro_rules! Depcrate_shims_unwindCatchUnwindData {
() => {
// Module: crate::shims::unwind
// Provides: {"CatchUnwindData"}
// Dependencies: {}
# [doc = " Holds all of the relevant data for when unwinding hits a `try` frame."] # [derive (Debug)] pub struct CatchUnwindData < 'tcx > { # [doc = " The `catch_fn` callback to call in case of a panic."] catch_fn : Pointer , # [doc = " The `data` argument for that callback."] data : ImmTy < 'tcx > , # [doc = " The return place from the original call to `try`."] dest : MPlaceTy < 'tcx > , # [doc = " The return block from the original call to `try`."] ret : Option < mir :: BasicBlock > , }
};
}
