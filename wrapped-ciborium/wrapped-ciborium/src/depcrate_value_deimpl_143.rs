// Generated macro for impl_143 (impl)
macro_rules! Depcrate_value_deimpl_143 {
() => {
// Module: crate::value::de
// Provides: {"impl_143"}
// Dependencies: {}
impl < 'a > From < Integer > for de :: Unexpected < 'a > { # [inline] fn from (value : Integer) -> Self { u64 :: try_from (value) . map (de :: Unexpected :: Unsigned) . unwrap_or_else (| _ | { i64 :: try_from (value) . map (de :: Unexpected :: Signed) . unwrap_or_else (| _ | de :: Unexpected :: Other ("large integer")) }) } }
};
}
