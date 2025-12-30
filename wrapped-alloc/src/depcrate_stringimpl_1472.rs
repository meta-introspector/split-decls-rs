// Generated macro for impl_1472 (impl)
macro_rules! Depcrate_stringimpl_1472 {
() => {
// Module: crate::string
// Provides: {"impl_1472"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : fmt :: Display + ? Sized > SpecToString for T { # [inline] default fn spec_to_string (& self) -> String { let mut buf = String :: new () ; let mut formatter = core :: fmt :: Formatter :: new (& mut buf , core :: fmt :: FormattingOptions :: new ()) ; fmt :: Display :: fmt (self , & mut formatter) . expect ("a Display implementation returned an error unexpectedly") ; buf } }
};
}
