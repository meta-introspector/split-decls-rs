// Generated macro for impl_85 (impl)
macro_rules! Depcrate_jsonimpl_85 {
() => {
// Module: crate::json
// Provides: {"impl_85"}
// Dependencies: {}
impl < W > JSON < W > { # [doc = " Returns true if and only if this printer has written at least one byte"] # [doc = " to the underlying writer during any of the previous searches."] pub fn has_written (& self) -> bool { self . wtr . total_count () > 0 } # [doc = " Return a mutable reference to the underlying writer."] pub fn get_mut (& mut self) -> & mut W { self . wtr . get_mut () } # [doc = " Consume this printer and return back ownership of the underlying"] # [doc = " writer."] pub fn into_inner (self) -> W { self . wtr . into_inner () } }
};
}
