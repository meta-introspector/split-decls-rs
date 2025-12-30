// Generated macro for impl_656 (impl)
macro_rules! Depcrate_options_shapeimpl_656 {
() => {
// Module: crate::options::shape
// Provides: {"impl_656"}
// Dependencies: {}
impl FromMeta for DataShape { fn from_list (items : & [NestedMeta]) -> Result < Self > { let mut errors = Error :: accumulator () ; let mut new = DataShape :: default () ; for item in items { if let NestedMeta :: Meta (Meta :: Path (ref path)) = * item { errors . handle (new . set_word (& path . segments . first () . unwrap () . ident . to_string ())) ; } else { errors . push (Error :: unsupported_format ("non-word") . with_span (item)) ; } } errors . finish_with (new) } }
};
}
