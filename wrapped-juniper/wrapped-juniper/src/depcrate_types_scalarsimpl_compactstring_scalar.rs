// Generated macro for impl_compactstring_scalar (module)
macro_rules! Depcrate_types_scalarsimpl_compactstring_scalar {
() => {
// Module: crate::types::scalars
// Provides: {"impl_compactstring_scalar"}
// Dependencies: {}
mod impl_compactstring_scalar { use super :: CompactString ; use crate :: { FromScalarValue , Scalar , ScalarValue } ; pub (super) fn from_input < S : ScalarValue > (v : & Scalar < S > ,) -> Result < CompactString , < & str as FromScalarValue < '_ , S > > :: Error > { if let Some (s) = v . downcast_type :: < CompactString > () { Ok (s . clone ()) } else { v . try_to :: < & str > () . map (CompactString :: from) } } }
};
}
