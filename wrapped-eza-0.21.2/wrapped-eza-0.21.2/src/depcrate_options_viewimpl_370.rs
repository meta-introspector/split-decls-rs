// Generated macro for impl_370 (impl)
macro_rules! Depcrate_options_viewimpl_370 {
() => {
// Module: crate::options::view
// Provides: {"impl_370"}
// Dependencies: {}
impl details :: Options { fn deduce_tree < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { let details = details :: Options { table : None , header : false , xattr : xattr :: ENABLED && matches . has (& flags :: EXTENDED) ? , secattr : xattr :: ENABLED && matches . has (& flags :: SECURITY_CONTEXT) ? , mounts : matches . has (& flags :: MOUNTS) ? , color_scale : ColorScaleOptions :: deduce (matches , vars) ? , follow_links : matches . has (& flags :: FOLLOW_LINKS) ? , } ; Ok (details) } fn deduce_long < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { if matches . is_strict () { if matches . has (& flags :: ACROSS) ? && ! matches . has (& flags :: GRID) ? { return Err (OptionsError :: Useless (& flags :: ACROSS , true , & flags :: LONG)) ; } else if matches . has (& flags :: ONE_LINE) ? { return Err (OptionsError :: Useless (& flags :: ONE_LINE , true , & flags :: LONG)) ; } } Ok (details :: Options { table : Some (TableOptions :: deduce (matches , vars) ?) , header : matches . has (& flags :: HEADER) ? , xattr : xattr :: ENABLED && matches . has (& flags :: EXTENDED) ? , secattr : xattr :: ENABLED && matches . has (& flags :: SECURITY_CONTEXT) ? , mounts : matches . has (& flags :: MOUNTS) ? , color_scale : ColorScaleOptions :: deduce (matches , vars) ? , follow_links : matches . has (& flags :: FOLLOW_LINKS) ? , }) } }
};
}
