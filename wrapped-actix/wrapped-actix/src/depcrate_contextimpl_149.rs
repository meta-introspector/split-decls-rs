// Generated macro for impl_149 (impl)
macro_rules! Depcrate_contextimpl_149 {
() => {
// Module: crate::context
// Provides: {"impl_149"}
// Dependencies: {}
impl < A > AsyncContext < A > for Context < A > where A : Actor < Context = Self > , { # [inline] fn spawn < F > (& mut self , fut : F) -> SpawnHandle where F : ActorFuture < A , Output = () > + 'static , { self . parts . spawn (fut) } # [inline] fn wait < F > (& mut self , fut : F) where F : ActorFuture < A , Output = () > + 'static , { self . parts . wait (fut) } # [inline] fn waiting (& self) -> bool { self . parts . waiting () } # [inline] fn cancel_future (& mut self , handle : SpawnHandle) -> bool { self . parts . cancel_future (handle) } # [inline] fn address (& self) -> Addr < A > { self . parts . address () } }
};
}
