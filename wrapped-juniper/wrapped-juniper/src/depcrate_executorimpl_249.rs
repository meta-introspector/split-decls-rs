// Generated macro for impl_249 (impl)
macro_rules! Depcrate_executorimpl_249 {
() => {
// Module: crate::executor
// Provides: {"impl_249"}
// Dependencies: {}
impl < T : Display , S > From < T > for FieldError < S > { fn from (e : T) -> Self { Self { message : e . to_string () , extensions : Value :: Null , } } }
};
}
