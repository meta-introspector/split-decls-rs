// Generated macro for impl_arcstr_scalar (module)
macro_rules! Depcrate_types_scalarsimpl_arcstr_scalar {
() => {
// Module: crate::types::scalars
// Provides: {"impl_arcstr_scalar"}
// Dependencies: {}
mod impl_arcstr_scalar { use super :: ArcStr ; use crate :: { FromScalarValue , Scalar , ScalarValue } ; pub (super) fn from_input < S : ScalarValue > (v : & Scalar < S > ,) -> Result < ArcStr , < & str as FromScalarValue < '_ , S > > :: Error > { if let Some (s) = v . downcast_type :: < ArcStr > () { Ok (s . clone ()) } else { v . try_to :: < & str > () . map (ArcStr :: from) } } }
};
}
