// Generated macro for impl_625 (impl)
macro_rules! Depcrate_types_scalarsimpl_625 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_625"}
// Dependencies: {}
impl ID { fn to_output (& self) -> & str { & self . 0 } fn from_input < S : ScalarValue > (v : & Scalar < S >) -> Result < Self , WrongInputScalarTypeError < '_ , S > > { v . try_to_string () . or_else (| | v . try_to_int () . as_ref () . map (ToString :: to_string)) . map (| s | Self (s . into ())) . ok_or_else (| | WrongInputScalarTypeError { type_name : arcstr :: literal ! ("String` or `Int") , input : & * * v , }) } }
};
}
