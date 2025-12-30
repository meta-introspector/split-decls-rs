// Generated macro for impl_97 (impl)
macro_rules! Depcrate_paserkimpl_97 {
() => {
// Module: crate::paserk
// Provides: {"impl_97"}
// Dependencies: {}
# [cfg (feature = "v4")] impl TryFrom < & str > for SymmetricKey < V4 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (Self { bytes : validate_paserk_string (value , "k4" , "local" , V4 :: LOCAL_KEY) ? , phantom : PhantomData , }) } }
};
}
