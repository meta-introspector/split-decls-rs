// Generated macro for test (module)
macro_rules! Depcrate_displaytest {
() => {
// Module: crate::display
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_display_count_with_point () { let result = CGDisplay :: display_count_with_point (CGPoint :: new (0. , 0.)) ; assert ! (result . is_ok ()) ; } # [test] fn test_displays_with_point_0 () { let result = CGDisplay :: displays_with_point (CGPoint :: new (0. , 0.) , 0) ; assert ! (result . is_ok ()) ; let (displays , count) = result . unwrap () ; assert_eq ! (displays . len () , count as usize) ; } # [test] fn test_displays_with_point_5 () { let result = CGDisplay :: displays_with_point (CGPoint :: new (0. , 0.) , 5) ; assert ! (result . is_ok ()) ; let (displays , count) = result . unwrap () ; assert_eq ! (displays . len () , count as usize) ; } # [test] fn test_display_count_with_rect () { let _ = CGDisplay :: main () ; let result = CGDisplay :: display_count_with_rect (CGRect :: new (& CGPoint :: new (10. , 10.) , & CGSize :: new (100. , 100.) ,)) ; assert ! (result . is_ok ()) ; } # [test] fn test_displays_with_rect_0 () { let _ = CGDisplay :: main () ; let result = CGDisplay :: displays_with_rect (CGRect :: new (& CGPoint :: new (0. , 0.) , & CGSize :: new (100. , 100.)) , 0 ,) ; assert ! (result . is_ok ()) ; let (displays , count) = result . unwrap () ; assert_eq ! (displays . len () , count as usize) ; } # [test] fn test_displays_with_rect_5 () { let _ = CGDisplay :: main () ; let result = CGDisplay :: displays_with_rect (CGRect :: new (& CGPoint :: new (0. , 0.) , & CGSize :: new (100. , 100.)) , 5 ,) ; assert ! (result . is_ok ()) ; let (displays , count) = result . unwrap () ; assert_eq ! (displays . len () , count as usize) ; } }
};
}
