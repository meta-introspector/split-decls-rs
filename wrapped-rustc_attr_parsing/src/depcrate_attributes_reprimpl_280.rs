// Generated macro for impl_280 (impl)
macro_rules! Depcrate_attributes_reprimpl_280 {
() => {
// Module: crate::attributes::repr
// Provides: {"impl_280"}
// Dependencies: {}
impl < S : Stage > CombineAttributeParser < S > for ReprParser { type Item = (ReprAttr , Span) ; const PATH : & [Symbol] = & [sym :: repr] ; const CONVERT : ConvertFn < Self :: Item > = | items , first_span | AttributeKind :: Repr { reprs : items , first_span } ; const TEMPLATE : AttributeTemplate = template ! (List : & ["C" , "Rust" , "transparent" , "align(...)" , "packed(...)" , "<integer type>"] , "https://doc.rust-lang.org/reference/type-layout.html#representations") ; fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > + 'c { let mut reprs = Vec :: new () ; let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return reprs ; } ; if list . is_empty () { cx . warn_empty_attribute (cx . attr_span) ; return reprs ; } for param in list . mixed () { if let Some (_) = param . lit () { cx . emit_err (session_diagnostics :: ReprIdent { span : cx . attr_span }) ; continue ; } reprs . extend (param . meta_item () . and_then (| mi | parse_repr (cx , & mi)) . map (| r | (r , param . span ())) ,) ; } reprs } const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; }
};
}
