// Generated macro for test (module)
macro_rules! Depcrate_element_pietest {
() => {
// Module: crate::element::pie
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn polar_coord_to_cartestian_coord () { let coord = theta_to_ordinal_coord (800.0 , 1.5_f64 . to_radians () , & (5 , 5)) ; assert_eq ! (coord , (805 , 26)) ; } # [test] fn pie_calculations () { let mut center = (5 , 5) ; let mut radius = 800.0 ; let sizes = vec ! [50.0 , 25.0] ; let colors = vec ! [] ; let labels : Vec < & str > = vec ! [] ; let pie = Pie :: new (& center , & radius , & sizes , & colors , & labels) ; assert_eq ! (pie . total , 75.0) ; center . 1 += 1 ; radius += 1.0 ; assert ! (colors . get (0) . is_none ()) ; assert ! (labels . first () . is_none ()) ; assert_eq ! (radius , 801.0) ; } }
};
}
