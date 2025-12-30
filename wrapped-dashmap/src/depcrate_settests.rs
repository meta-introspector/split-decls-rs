// Generated macro for tests (module)
macro_rules! Depcrate_settests {
() => {
// Module: crate::set
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: DashSet ; # [test] fn test_basic () { let set = DashSet :: new () ; set . insert (0) ; assert_eq ! (set . get (& 0) . as_deref () , Some (& 0)) ; } # [test] fn test_default () { let set : DashSet < u32 > = DashSet :: default () ; set . insert (0) ; assert_eq ! (set . get (& 0) . as_deref () , Some (& 0)) ; } # [test] fn test_equal () { let set1 = DashSet :: new () ; let set2 = DashSet :: new () ; assert_eq ! (set1 , set2) ; set1 . insert ("Hello, world!") ; assert_ne ! (set1 , set2) ; set1 . insert ("Goodbye, world!") ; assert_ne ! (set1 , set2) ; set2 . insert ("Hello, world!") ; assert_ne ! (set1 , set2) ; set2 . insert ("Goodbye, world!") ; assert_eq ! (set1 , set2) ; } # [test] fn test_multiple_hashes () { let set = DashSet :: < u32 > :: default () ; for i in 0 .. 100 { assert ! (set . insert (i)) ; } for i in 0 .. 100 { assert ! (! set . insert (i)) ; } for i in 0 .. 100 { assert_eq ! (Some (i) , set . remove (& i)) ; } for i in 0 .. 100 { assert_eq ! (None , set . remove (& i)) ; } } }
};
}
