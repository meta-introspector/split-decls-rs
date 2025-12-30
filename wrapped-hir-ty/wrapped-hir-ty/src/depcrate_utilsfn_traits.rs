// Generated macro for fn_traits (function)
macro_rules! Depcrate_utilsfn_traits {
() => {
// Module: crate::utils
// Provides: {"fn_traits"}
// Dependencies: {}
pub (crate) fn fn_traits (db : & dyn DefDatabase , krate : Crate) -> impl Iterator < Item = TraitId > + '_ { [LangItem :: Fn , LangItem :: FnMut , LangItem :: FnOnce] . into_iter () . filter_map (move | lang | lang . resolve_trait (db , krate)) }
};
}
