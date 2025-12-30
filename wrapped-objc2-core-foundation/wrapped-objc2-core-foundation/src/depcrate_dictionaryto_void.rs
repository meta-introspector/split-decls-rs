// Generated macro for to_void (function)
macro_rules! Depcrate_dictionaryto_void {
() => {
// Module: crate::dictionary
// Provides: {"to_void"}
// Dependencies: {}
# [doc = " These usually doesn't _have_ to be bound by `K: Type`, all that matters is"] # [doc = " that they're valid for the dictionary at hand."] # [doc = ""] # [doc = " But let's keep the bound for now, it might turn out to be necessary."] # [inline] fn to_void < K : ? Sized + Type > (key : & K) -> * const c_void { let key : * const K = key ; key . cast () }
};
}
