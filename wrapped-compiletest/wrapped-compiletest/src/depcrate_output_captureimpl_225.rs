// Generated macro for impl_225 (impl)
macro_rules! Depcrate_output_captureimpl_225 {
() => {
// Module: crate::output_capture
// Provides: {"impl_225"}
// Dependencies: {}
impl ConsoleOut for CaptureBuf { fn write_fmt (& self , args : fmt :: Arguments < '_ >) { let mut s = self . inner . lock () . unwrap_or_else (| e | e . into_inner ()) ; < String as fmt :: Write > :: write_fmt (& mut s , args) . unwrap () ; } }
};
}
