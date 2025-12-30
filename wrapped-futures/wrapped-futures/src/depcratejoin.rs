// Generated macro for join (macro)
macro_rules! Depcratejoin {
() => {
// Module: crate
// Provides: {"join"}
// Dependencies: {}
macro_rules ! join { ($ (($ join : ident , $ Join : ident , $ new : ident , <$ ($ name : ident : $ B : ident) ,*>) ,) *) => ($ (# [doc = " Same as `join`, but with more futures."] fn $ join <$ ($ B) ,*> (self , $ ($ name : $ B) ,*) -> $ Join < Self , $ ($ B :: Future) ,*> where $ ($ B : IntoFuture < Error = Self :: Error >,) * Self : Sized , { join ::$ new (self , $ ($ name . into_future ()) ,*) }) *) }
};
}
