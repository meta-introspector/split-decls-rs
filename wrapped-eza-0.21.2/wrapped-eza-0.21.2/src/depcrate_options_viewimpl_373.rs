// Generated macro for impl_373 (impl)
macro_rules! Depcrate_options_viewimpl_373 {
() => {
// Module: crate::options::view
// Provides: {"impl_373"}
// Dependencies: {}
impl TableOptions { fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { let time_format = TimeFormat :: deduce (matches , vars) ? ; let size_format = SizeFormat :: deduce (matches) ? ; let user_format = UserFormat :: deduce (matches) ? ; let group_format = GroupFormat :: deduce (matches) ? ; let flags_format = FlagsFormat :: deduce (vars) ; let columns = Columns :: deduce (matches , vars) ? ; Ok (Self { size_format , time_format , user_format , group_format , flags_format , columns , }) } }
};
}
