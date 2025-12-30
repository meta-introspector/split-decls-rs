// Generated macro for Factory (struct)
macro_rules! Depcrate_factoryFactory {
() => {
// Module: crate::factory
// Provides: {"Factory"}
// Dependencies: {}
# [doc = " The factory is used to create object paths, interfaces, methods etc."] # [doc = ""] # [doc = " There are three factories:"] # [doc = ""] # [doc = "  **MTFn** - all methods are `Fn()`."] # [doc = ""] # [doc = "  **MTFnMut** - all methods are `FnMut()`. This means they can mutate their environment,"] # [doc = "  which has the side effect that if you call it recursively, it will RefCell panic."] # [doc = ""] # [doc = "  **MTSync** - all methods are `Fn() + Send + Sync + 'static`. This means that the methods"] # [doc = "  can be called from different threads in parallel."] # [doc = ""] # [derive (Debug , Clone)] pub struct Factory < M : MethodType < D > , D : DataType = () > (Arc < IfaceCache < M , D > >) ;
};
}
