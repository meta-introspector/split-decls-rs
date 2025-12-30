// Generated macro for impl_74 (impl)
macro_rules! Depcrate_static_com_objectimpl_74 {
() => {
// Module: crate::static_com_object
// Provides: {"impl_74"}
// Dependencies: {}
impl INumberFactory_Impl for MyFactory_Impl { unsafe fn next (& self) -> u32 { self . x . fetch_add (1 , SeqCst) } unsafe fn add (& self , x : u32 , y : u32) -> u32 { x + y } }
};
}
