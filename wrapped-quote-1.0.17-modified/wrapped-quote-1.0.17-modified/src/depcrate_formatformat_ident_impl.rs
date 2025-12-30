// Generated macro for format_ident_impl (macro)
macro_rules! Depcrate_formatformat_ident_impl {
() => {
// Module: crate::format
// Provides: {"format_ident_impl"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! format_ident_impl { ([$ span : expr , $ ($ fmt : tt) *]) => { $ crate :: __private :: mk_ident (& format ! ($ ($ fmt) *) , $ span) } ; ([$ old : expr , $ ($ fmt : tt) *] span = $ span : expr) => { $ crate :: format_ident_impl ! ([$ old , $ ($ fmt) *] span = $ span ,) } ; ([$ old : expr , $ ($ fmt : tt) *] span = $ span : expr , $ ($ rest : tt) *) => { $ crate :: format_ident_impl ! ([:: std :: option :: Option :: Some ::<$ crate :: __private :: Span > ($ span) , $ ($ fmt) *] $ ($ rest) *) } ; ([$ span : expr , $ ($ fmt : tt) *] $ name : ident = $ arg : expr) => { $ crate :: format_ident_impl ! ([$ span , $ ($ fmt) *] $ name = $ arg ,) } ; ([$ span : expr , $ ($ fmt : tt) *] $ name : ident = $ arg : expr , $ ($ rest : tt) *) => { match $ crate :: __private :: IdentFragmentAdapter (&$ arg) { arg => $ crate :: format_ident_impl ! ([$ span . or (arg . span ()) , $ ($ fmt) *, $ name = arg] $ ($ rest) *) , } } ; ([$ span : expr , $ ($ fmt : tt) *] $ arg : expr) => { $ crate :: format_ident_impl ! ([$ span , $ ($ fmt) *] $ arg ,) } ; ([$ span : expr , $ ($ fmt : tt) *] $ arg : expr , $ ($ rest : tt) *) => { match $ crate :: __private :: IdentFragmentAdapter (&$ arg) { arg => $ crate :: format_ident_impl ! ([$ span . or (arg . span ()) , $ ($ fmt) *, arg] $ ($ rest) *) , } } ; }
};
}
