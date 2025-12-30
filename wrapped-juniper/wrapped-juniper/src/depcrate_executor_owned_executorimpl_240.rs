// Generated macro for impl_240 (impl)
macro_rules! Depcrate_executor_owned_executorimpl_240 {
() => {
// Module: crate::executor::owned_executor
// Provides: {"impl_240"}
// Dependencies: {}
impl < 'a , CtxT , S > OwnedExecutor < 'a , CtxT , S > { # [doc (hidden)] pub fn fragment_by_name < 'b > (& 'b self , name : & str) -> Option < & 'b Fragment < 'a , S > > { self . fragments . get (name) } # [doc (hidden)] pub fn context (& self) -> & 'a CtxT { self . context } # [doc (hidden)] pub fn schema (& self) -> & 'a SchemaType < S > { self . schema } # [doc (hidden)] pub fn location (& self) -> & SourcePosition { self . field_path . location () } }
};
}
