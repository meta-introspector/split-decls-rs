// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl IFrameworkView_Impl for CoreAppView_Impl { fn Initialize (& self , _ : Ref < CoreApplicationView >) -> Result < () > { Ok (()) } fn Load (& self , _ : & HSTRING) -> Result < () > { Ok (()) } fn Uninitialize (& self) -> Result < () > { Ok (()) } fn Run (& self) -> Result < () > { let window = CoreWindow :: GetForCurrentThread () ? ; window . Activate () ? ; let dispatcher = window . Dispatcher () ? ; dispatcher . ProcessEvents (CoreProcessEventsOption :: ProcessUntilQuit) ? ; Ok (()) } fn SetWindow (& self , _ : Ref < CoreWindow >) -> Result < () > { Ok (()) } }
};
}
