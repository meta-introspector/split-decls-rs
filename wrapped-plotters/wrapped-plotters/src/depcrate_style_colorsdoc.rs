// Generated macro for doc (macro)
macro_rules! Depcrate_style_colorsdoc {
() => {
// Module: crate::style::colors
// Provides: {"doc"}
// Dependencies: {}
# [doc = " Macro for allowing dynamic creation of doc attributes."] # [macro_export] macro_rules ! doc { { $ (# [$ m : meta]) * $ ([$ doc : expr] $ (# [$ n : meta]) *) * @ $ thing : item } => { $ (# [$ m]) * $ (# [doc = $ doc] $ (# [$ n]) *) * $ thing } }
};
}
