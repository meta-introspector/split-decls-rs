// Generated macro for impl_344 (impl)
macro_rules! Depcrate_attributes_traitsimpl_344 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_344"}
// Dependencies: {}
impl < S : Stage > SingleAttributeParser < S > for SkipDuringMethodDispatchParser { const PATH : & [Symbol] = & [sym :: rustc_skip_during_method_dispatch] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const TEMPLATE : AttributeTemplate = template ! (List : & ["array, boxed_slice"]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let mut array = false ; let mut boxed_slice = false ; let Some (args) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; if args . is_empty () { cx . expected_at_least_one_argument (args . span) ; return None ; } for arg in args . mixed () { let Some (arg) = arg . meta_item () else { cx . unexpected_literal (arg . span ()) ; continue ; } ; if let Err (span) = arg . args () . no_args () { cx . expected_no_args (span) ; } let path = arg . path () ; let (key , skip) : (Symbol , & mut bool) = match path . word_sym () { Some (key @ sym :: array) => (key , & mut array) , Some (key @ sym :: boxed_slice) => (key , & mut boxed_slice) , _ => { cx . expected_specific_argument (path . span () , & [sym :: array , sym :: boxed_slice]) ; continue ; } } ; if mem :: replace (skip , true) { cx . duplicate_key (arg . span () , key) ; } } Some (AttributeKind :: SkipDuringMethodDispatch { array , boxed_slice , span : cx . attr_span }) } }
};
}
