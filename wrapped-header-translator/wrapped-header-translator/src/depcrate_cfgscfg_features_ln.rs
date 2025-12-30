// Generated macro for cfg_features_ln (function)
macro_rules! Depcrate_cfgscfg_features_ln {
() => {
// Module: crate::cfgs
// Provides: {"cfg_features_ln"}
// Dependencies: {}
pub (crate) fn cfg_features_ln < 'a , I , F > (feature_names : I) -> impl Display + 'a where I : IntoIterator < Item = F > + Clone + 'a , F : AsRef < str > , { FormatterFn (move | f | { let mut iter = feature_names . clone () . into_iter () . peekable () ; if let Some (first) = iter . next () { if iter . peek () . is_none () { writeln ! (f , "#[cfg(feature = {:?})]" , first . as_ref ()) ? ; } else { write ! (f , "#[cfg(all(") ? ; write ! (f , "feature = {:?}" , first . as_ref ()) ? ; for feature in iter { write ! (f , ", feature = {:?}" , feature . as_ref ()) ? ; } write ! (f , "))]") ? ; writeln ! (f) ? ; } } else { } Ok (()) }) }
};
}
