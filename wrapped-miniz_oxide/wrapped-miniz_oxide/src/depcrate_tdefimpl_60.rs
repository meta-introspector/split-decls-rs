// Generated macro for impl_60 (impl)
macro_rules! Depcrate_tdefimpl_60 {
() => {
// Module: crate::tdef
// Provides: {"impl_60"}
// Dependencies: {}
# [doc = " Convert an i32 to a TDEFLFlush"] # [doc = ""] # [doc = " Returns TDEFLFLush::None flush value is unknown."] # [doc = " For use with c interface."] impl Compressor { pub (crate) fn new_with_callback (flags : u32 , func : CallbackFunc) -> Self { Compressor { inner : Some (CompressorOxide :: new (flags)) , callback : Some (func) , } } # [doc = " Sets the inner state to `None` and thus drops it."] pub fn drop_inner (& mut self) { self . inner = None ; } # [doc = " Reset the inner compressor if any."] pub fn reset (& mut self) { if let Some (c) = self . inner . as_mut () { c . reset () ; } } pub fn adler32 (& self) -> u32 { self . inner . as_ref () . map (| i | i . adler32 ()) . unwrap_or (0) } pub fn prev_return_status (& self) -> TDEFLStatus { self . inner . as_ref () . map (| i | i . prev_return_status ()) . unwrap_or (TDEFLStatus :: BadParam) } # [doc = " Return the compressor flags of the inner compressor."] pub fn flags (& self) -> i32 { self . inner . as_ref () . map (| i | i . flags ()) . unwrap_or (0) } }
};
}
