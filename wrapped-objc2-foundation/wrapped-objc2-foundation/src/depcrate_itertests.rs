// Generated macro for tests (module)
macro_rules! Depcrate_itertests {
() => {
// Module: crate::iter
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "NSArray")] # [cfg (feature = "NSValue")] mod tests { use alloc :: vec :: Vec ; use core :: mem :: size_of ; use super :: * ; use crate :: { NSArray , NSNumber } ; # [test] # [cfg_attr (any (not (target_pointer_width = "64") , debug_assertions) , ignore = "assertions assume pointer-width of 64, and the size only really matter in release mode")] fn test_enumerator_helper () { assert_eq ! (size_of ::< NSFastEnumerationState > () , 64) ; assert_eq ! (size_of ::< FastEnumeratorHelper > () , 208) ; assert_eq ! (size_of ::< IterUnchecked <'_ , NSArray < NSNumber >>> () , 216) ; assert_eq ! (size_of ::< Iter <'_ , NSArray < NSNumber >>> () , 232) ; assert_eq ! (size_of ::< IntoIter < NSArray < NSNumber >>> () , 232) ; } # [test] fn test_enumerator () { let vec : Vec < _ > = (0 .. 4) . map (NSNumber :: new_usize) . collect () ; let array = NSArray :: from_retained_slice (& vec) ; let enumerator = array . iter () ; assert_eq ! (enumerator . count () , 4) ; let enumerator = array . iter () ; assert ! (enumerator . enumerate () . all (| (i , obj) | obj . as_usize () == i)) ; } # [test] fn test_into_enumerator () { let vec : Vec < _ > = (0 .. 4) . map (NSNumber :: new_usize) . collect () ; let array = NSArray :: from_retained_slice (& vec) ; let enumerator = array . into_iter () ; assert ! (enumerator . enumerate () . all (| (i , obj) | obj . as_usize () == i)) ; } }
};
}
