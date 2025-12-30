// Generated macro for impl_94 (impl)
macro_rules! Depcrate_builder_argimpl_94 {
() => {
// Module: crate::builder::arg
// Provides: {"impl_94"}
// Dependencies: {}
impl fmt :: Debug for Arg { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { let mut ds = f . debug_struct ("Arg") ; # [allow (unused_mut)] let mut ds = ds . field ("id" , & self . id) . field ("help" , & self . help) . field ("long_help" , & self . long_help) . field ("action" , & self . action) . field ("value_parser" , & self . value_parser) . field ("blacklist" , & self . blacklist) . field ("settings" , & self . settings) . field ("overrides" , & self . overrides) . field ("groups" , & self . groups) . field ("requires" , & self . requires) . field ("r_ifs" , & self . r_ifs) . field ("r_unless" , & self . r_unless) . field ("short" , & self . short) . field ("long" , & self . long) . field ("aliases" , & self . aliases) . field ("short_aliases" , & self . short_aliases) . field ("disp_ord" , & self . disp_ord) . field ("val_names" , & self . val_names) . field ("num_vals" , & self . num_vals) . field ("val_delim" , & self . val_delim) . field ("default_vals" , & self . default_vals) . field ("default_vals_ifs" , & self . default_vals_ifs) . field ("terminator" , & self . terminator) . field ("index" , & self . index) . field ("help_heading" , & self . help_heading) . field ("default_missing_vals" , & self . default_missing_vals) . field ("ext" , & self . ext) ; # [cfg (feature = "env")] { ds = ds . field ("env" , & self . env) ; } ds . finish () } }
};
}
