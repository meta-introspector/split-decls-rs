// Generated macro for impl_serde (function)
macro_rules! Depcrate_item_enumimpl_serde {
() => {
// Module: crate::item_enum
// Provides: {"impl_serde"}
// Dependencies: {}
fn impl_serde (ident : & syn :: Ident , variants : & Variants) -> TokenStream { let arms = fold_quote (variants . iter () , | v | { let v_ident = & v . ident ; let pattern = match v . fields { syn :: Fields :: Named (..) => quote ! (# ident :: v_ident { .. }) , syn :: Fields :: Unnamed (..) => quote ! (# ident ::# v_ident (..)) , syn :: Fields :: Unit => quote ! (# ident ::# v_ident) , } ; let option_value = config_value_of_variant (v) ; quote ! { # pattern => serializer . serialize_str (&# option_value) , } }) ; quote ! { impl :: serde :: ser :: Serialize for # ident { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer , { use serde :: ser :: Error ; match self { # arms _ => Err (S :: Error :: custom (format ! ("Cannot serialize {:?}" , self))) , } } } } }
};
}
