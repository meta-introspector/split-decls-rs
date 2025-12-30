// Generated macro for Store (struct)
macro_rules! Depcrate_storeStore {
() => {
// Module: crate::store
// Provides: {"Store"}
// Dependencies: {}
# [doc = " A combinator which will store some data into task-local storage."] # [doc = ""] # [doc = " This combinator is created by the `futures::store` method."] pub struct Store < T : Send + 'static , E > { item : Option < T > , _marker : marker :: PhantomData < fn () -> E > , }
};
}
