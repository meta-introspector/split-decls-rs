// Generated macro for lit_suffix_length (function)
macro_rules! Depcrate_numeric_literallit_suffix_length {
() => {
// Module: crate::numeric_literal
// Provides: {"lit_suffix_length"}
// Dependencies: {}
fn lit_suffix_length (lit_kind : & LitKind) -> Option < usize > { debug_assert ! (lit_kind . is_numeric ()) ; let suffix = match lit_kind { LitKind :: Int (_ , int_lit_kind) => match int_lit_kind { LitIntType :: Signed (int_ty) => Some (int_ty . name_str ()) , LitIntType :: Unsigned (uint_ty) => Some (uint_ty . name_str ()) , LitIntType :: Unsuffixed => None , } , LitKind :: Float (_ , float_lit_kind) => match float_lit_kind { LitFloatType :: Suffixed (float_ty) => Some (float_ty . name_str ()) , LitFloatType :: Unsuffixed => None , } , _ => None , } ; suffix . map (str :: len) }
};
}
