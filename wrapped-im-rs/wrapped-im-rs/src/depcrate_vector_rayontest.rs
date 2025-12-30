// Generated macro for test (module)
macro_rules! Depcrate_vector_rayontest {
() => {
// Module: crate::vector::rayon
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: super :: * ; use super :: proptest :: vector ; use :: proptest :: num :: i32 ; use :: proptest :: proptest ; use :: rayon :: iter :: { IntoParallelRefIterator , IntoParallelRefMutIterator , ParallelIterator } ; proptest ! { # [test] fn par_iter (ref mut input in vector (i32 :: ANY , 0 .. 10000)) { assert_eq ! (input . iter () . max () , input . par_iter () . max ()) } # [test] fn par_mut_iter (ref mut input in vector (i32 :: ANY , 0 .. 10000)) { let mut vec = input . clone () ; vec . par_iter_mut () . for_each (| i | * i = i . overflowing_add (1) . 0) ; let expected : Vector < i32 > = input . clone () . into_iter () . map (| i | i . overflowing_add (1) . 0) . collect () ; assert_eq ! (expected , vec) ; } } }
};
}
