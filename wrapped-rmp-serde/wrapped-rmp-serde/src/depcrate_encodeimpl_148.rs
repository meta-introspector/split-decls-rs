// Generated macro for impl_148 (impl)
macro_rules! Depcrate_encodeimpl_148 {
() => {
// Module: crate::encode
// Provides: {"impl_148"}
// Dependencies: {}
impl < W : Write , C > Serializer < W , C > { # [doc = " Gets a reference to the underlying writer."] # [inline (always)] pub fn get_ref (& self) -> & W { & self . wr } # [doc = " Gets a mutable reference to the underlying writer."] # [doc = ""] # [doc = " It is inadvisable to directly write to the underlying writer."] # [inline (always)] pub fn get_mut (& mut self) -> & mut W { & mut self . wr } # [doc = " Unwraps this `Serializer`, returning the underlying writer."] # [inline (always)] pub fn into_inner (self) -> W { self . wr } # [doc = " Changes the maximum nesting depth that is allowed."] # [doc = ""] # [doc = " Currently unused."] # [doc (hidden)] # [inline] pub fn unstable_set_max_depth (& mut self , depth : usize) { self . depth = depth . min (u16 :: MAX as _) as u16 ; } }
};
}
