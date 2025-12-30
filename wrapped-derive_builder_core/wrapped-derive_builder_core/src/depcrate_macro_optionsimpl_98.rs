// Generated macro for impl_98 (impl)
macro_rules! Depcrate_macro_optionsimpl_98 {
() => {
// Module: crate::macro_options
// Provides: {"impl_98"}
// Dependencies: {}
impl FromMeta for VisibilityAttr { fn from_list (items : & [darling :: ast :: NestedMeta]) -> darling :: Result < Self > { # [derive (FromMeta)] struct VisibilityAttrInternal { public : Flag , private : Flag , vis : Option < syn :: Visibility > , } let VisibilityAttrInternal { public , private , vis : explicit , } = VisibilityAttrInternal :: from_list (items) ? ; let mut conflicts = Error :: accumulator () ; if public . is_present () { if private . is_present () { conflicts . push (Error :: custom ("`public` and `private` cannot be used together") . with_span (& private . span ()) ,) ; } if let Some (vis) = explicit { conflicts . push (Error :: custom ("`public` and `vis` cannot be used together") . with_span (& vis) ,) ; } conflicts . finish_with (Self :: Public (public . span ())) } else if let Some (vis) = explicit { if private . is_present () { conflicts . push (Error :: custom ("`vis` and `private` cannot be used together")) ; } conflicts . finish_with (Self :: Explicit (vis)) } else if private . is_present () { conflicts . finish_with (Self :: Private) } else { conflicts . finish_with (Self :: None) } } }
};
}
