// Generated macro for link (function)
macro_rules! Depcrate_slllink {
() => {
// Module: crate::sll
// Provides: {"link"}
// Dependencies: {}
# [cold] pub (crate) fn link < 'a , E : Elem > (head : & 'a Cell < * const E > , elem : & E) -> AddToSllResult < 'a , E > { let old_head = head . get () ; if old_head . is_null () { return AddToSllResult :: EmptyHead (head) ; } unsafe { if elem . key () < (* old_head) . key () { return AddToSllResult :: SmallerThanHead (head) ; } let mut curr = (* old_head) . prev () . get () ; loop { match (* curr) . key () . cmp (elem . key ()) { Ordering :: Less => return AddToSllResult :: SmallerThanNotHead (curr) , Ordering :: Equal => return AddToSllResult :: AlreadyInSll (curr) , Ordering :: Greater => curr = (* curr) . prev () . get () , } } } }
};
}
