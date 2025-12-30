// Generated macro for impl_530 (impl)
macro_rules! Depcrate_encodeimpl_530 {
() => {
// Module: crate::encode
// Provides: {"impl_530"}
// Dependencies: {}
unsafe impl < T : Encode , const LENGTH : usize > Encode for [T ; LENGTH] { const ENCODING : Encoding = Encoding :: Array (LENGTH as u64 , & T :: ENCODING) ; }
};
}
