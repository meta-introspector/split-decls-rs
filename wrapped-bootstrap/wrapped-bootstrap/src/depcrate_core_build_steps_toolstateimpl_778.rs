// Generated macro for impl_778 (impl)
macro_rules! Depcrate_core_build_steps_toolstateimpl_778 {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"impl_778"}
// Dependencies: {}
impl fmt :: Display for ToolState { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , match self { ToolState :: TestFail => "test-fail" , ToolState :: TestPass => "test-pass" , ToolState :: BuildFail => "build-fail" , }) } }
};
}
