// Generated macro for impl_676 (impl)
macro_rules! Depcrate_provider_pattern_runtime_displayimpl_676 {
() => {
// Module: crate::provider::pattern::runtime::display
// Provides: {"impl_676"}
// Dependencies: {}
impl Writeable for GenericPattern < '_ > { fn write_to < W : Write + ? Sized > (& self , formatter : & mut W) -> fmt :: Result { let mut buffer = alloc :: string :: String :: new () ; for pattern_item in self . items . iter () { match pattern_item { GenericPatternItem :: Placeholder (idx) => { dump_buffer_into_formatter (& buffer , formatter) ? ; buffer . clear () ; write ! (formatter , "{{{idx}}}") ? ; } GenericPatternItem :: Literal (ch) => { buffer . push (ch) ; } } } dump_buffer_into_formatter (& buffer , formatter) ? ; buffer . clear () ; Ok (()) } }
};
}
