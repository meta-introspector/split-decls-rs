// Generated macro for impl_14 (impl)
macro_rules! Depcrate_argsimpl_14 {
() => {
// Module: crate::args
// Provides: {"impl_14"}
// Dependencies: {}
impl FromMeta for Visible { fn from_value (value : & Lit) -> darling :: Result < Self > { match value { Lit :: Bool (LitBool { value : true , .. }) => Ok (Visible :: None) , Lit :: Bool (LitBool { value : false , .. }) => Ok (Visible :: HiddenAlways) , Lit :: Str (str) => Ok (Visible :: FnName (syn :: parse_str :: < Path > (& str . value ()) ?)) , _ => Err (darling :: Error :: unexpected_lit_type (value)) , } } }
};
}
