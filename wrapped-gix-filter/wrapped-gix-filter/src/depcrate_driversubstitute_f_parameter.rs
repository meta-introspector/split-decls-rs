// Generated macro for substitute_f_parameter (function)
macro_rules! Depcrate_driversubstitute_f_parameter {
() => {
// Module: crate::driver
// Provides: {"substitute_f_parameter"}
// Dependencies: {}
# [doc = " Substitute `path` as shell-save version into `cmd` which could be something like `cmd something %f`."] fn substitute_f_parameter (cmd : & BStr , path : & BStr) -> BString { let mut buf : BString = Vec :: with_capacity (cmd . len ()) . into () ; let mut ofs = 0 ; while let Some (pos) = cmd [ofs ..] . find (b"%f") { buf . push_str (& cmd [.. ofs + pos]) ; buf . extend_from_slice (& gix_quote :: single (path)) ; ofs += pos + 2 ; } buf . push_str (& cmd [ofs ..]) ; buf }
};
}
