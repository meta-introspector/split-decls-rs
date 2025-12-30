// Generated macro for impl_98 (impl)
macro_rules! Depcrate_frontendimpl_98 {
() => {
// Module: crate::frontend
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a , B , P > TryWriteable for WriteablePattern < 'a , B , P > where B : PatternBackend , P : PlaceholderValueProvider < B :: PlaceholderKey < 'a > , Error = B :: Error < 'a > > , { type Error = B :: Error < 'a > ; fn try_write_to_parts < S : PartsWrite + ? Sized > (& self , sink : & mut S ,) -> Result < Result < () , Self :: Error > , fmt :: Error > { let mut error = None ; let it = B :: iter_items (self . store) ; # [cfg (debug_assertions)] let (size_hint , mut actual_len) = (it . size_hint () , 0) ; for item in it { match item { PatternItem :: Literal (s) => { self . value_provider . map_literal (s) . write_to_parts (sink) ? ; } PatternItem :: Placeholder (key) => { let element_writeable = self . value_provider . value_for (key) ; if let Err (e) = element_writeable . try_write_to_parts (sink) ? { error . get_or_insert (e) ; } } } # [cfg (debug_assertions)] { actual_len += 1 ; } } # [cfg (debug_assertions)] { debug_assert ! (actual_len >= size_hint . 0) ; if let Some (max_len) = size_hint . 1 { debug_assert ! (actual_len <= max_len) ; } } if let Some (e) = error { Ok (Err (e)) } else { Ok (Ok (())) } } }
};
}
