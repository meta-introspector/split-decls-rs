// Generated macro for impl_236 (impl)
macro_rules! Depcrate_options_file_nameimpl_236 {
() => {
// Module: crate::options::file_name
// Provides: {"impl_236"}
// Dependencies: {}
impl Options { pub fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V , is_a_tty : bool ,) -> Result < Self , OptionsError > { let classify = Classify :: deduce (matches) ? ; let show_icons = ShowIcons :: deduce (matches , vars) ? ; let quote_style = QuoteStyle :: deduce (matches) ? ; let embed_hyperlinks = EmbedHyperlinks :: deduce (matches) ? ; let absolute = Absolute :: deduce (matches) ? ; Ok (Self { classify , show_icons , quote_style , embed_hyperlinks , absolute , is_a_tty , }) } }
};
}
