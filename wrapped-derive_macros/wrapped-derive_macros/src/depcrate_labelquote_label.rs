// Generated macro for quote_label (function)
macro_rules! Depcrate_labelquote_label {
() => {
// Module: crate::label
// Provides: {"quote_label"}
// Dependencies: {}
pub (crate) fn quote_label (label : Label) -> proc_macro2 :: TokenStream { match label { Label :: Implicit (implicit) => { quote ! (& sval :: Label :: new (# implicit) . with_tag (& sval :: tags :: VALUE_IDENT)) } Label :: Const (explicit) => quote ! (& sval :: Label :: new (# explicit)) , Label :: Ident (explicit) => quote ! (&# explicit) , } }
};
}
