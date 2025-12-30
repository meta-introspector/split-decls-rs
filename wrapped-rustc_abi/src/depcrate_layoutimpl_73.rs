// Generated macro for impl_73 (impl)
macro_rules! Depcrate_layoutimpl_73 {
() => {
// Module: crate::layout
// Provides: {"impl_73"}
// Dependencies: {}
impl < F > LayoutCalculatorError < F > { pub fn without_payload (& self) -> LayoutCalculatorError < () > { use LayoutCalculatorError :: * ; match * self { UnexpectedUnsized (_) => UnexpectedUnsized (()) , SizeOverflow => SizeOverflow , EmptyUnion => EmptyUnion , ReprConflict => ReprConflict , ZeroLengthSimdType => ZeroLengthSimdType , OversizedSimdType { max_lanes } => OversizedSimdType { max_lanes } , NonPrimitiveSimdType (_) => NonPrimitiveSimdType (()) , } } # [doc = " Format an untranslated diagnostic for this type"] # [doc = ""] # [doc = " Intended for use by rust-analyzer, as neither it nor `rustc_abi` depend on fluent infra."] pub fn fallback_fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use LayoutCalculatorError :: * ; f . write_str (match self { UnexpectedUnsized (_) => "an unsized type was found where a sized type was expected" , SizeOverflow => "size overflow" , EmptyUnion => "type is a union with no fields" , ReprConflict => "type has an invalid repr" , ZeroLengthSimdType | OversizedSimdType { .. } | NonPrimitiveSimdType (_) => { "invalid simd type definition" } }) } }
};
}
