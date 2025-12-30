// Generated macro for unlink (function)
macro_rules! Depcrate_sllunlink {
() => {
// Module: crate::sll
// Provides: {"unlink"}
// Dependencies: {}
# [cold] pub (crate) fn unlink < E : Elem > (head : & Cell < * const E > , elem : & E) { debug_assert ! (! head . get () . is_null () , "invalid linked list head") ; let elem_ptr : * const E = elem ; let prev = elem . prev () . replace (elem_ptr) ; let next = elem . next () . replace (elem_ptr) ; unsafe { debug_assert_eq ! ((* prev) . next () . get () , elem_ptr , "invalid linked list links") ; debug_assert_eq ! ((* next) . prev () . get () , elem_ptr , "invalid linked list links") ; (* prev) . next () . set (next) ; (* next) . prev () . set (prev) ; } if head . get () == elem_ptr { head . set (if next == elem_ptr { ptr :: null () } else { next }) } }
};
}
