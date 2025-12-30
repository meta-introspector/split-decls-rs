// Generated macro for width_unit_test (module)
macro_rules! Depcrate_output_cellwidth_unit_test {
() => {
// Module: crate::output::cell
// Provides: {"width_unit_test"}
// Dependencies: {}
# [cfg (test)] mod width_unit_test { use super :: DisplayWidth ; # [test] fn empty_string () { let cell = DisplayWidth :: from ("") ; assert_eq ! (* cell , 0) ; } # [test] fn test_string () { let cell = DisplayWidth :: from ("Diss Playwidth") ; assert_eq ! (* cell , 14) ; } # [test] fn addition () { let cell_one = DisplayWidth :: from ("/usr/bin/") ; let cell_two = DisplayWidth :: from ("drinking") ; assert_eq ! (* (cell_one + cell_two) , 17) ; } # [test] fn addition_usize () { let cell = DisplayWidth :: from ("/usr/bin/") ; assert_eq ! (* (cell + 8) , 17) ; } }
};
}
