macro_rules! deps {
    () => {
        Elem!();
        AddToSllResult!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < E : Elem > AddToSllResult < '_ , E > { pub (crate) fn add_to_sll (& self , elem_ptr : * const E) { unsafe { (* elem_ptr) . prev () . set (elem_ptr) ; (* elem_ptr) . next () . set (elem_ptr) ; match self { AddToSllResult :: EmptyHead (head) => head . set (elem_ptr) , AddToSllResult :: SmallerThanHead (head) => { let old_head = head . get () ; let prev = (* old_head) . prev () . replace (elem_ptr) ; (* prev) . next () . set (elem_ptr) ; (* elem_ptr) . next () . set (old_head) ; (* elem_ptr) . prev () . set (prev) ; head . set (elem_ptr) ; } AddToSllResult :: SmallerThanNotHead (curr) => { let next = (* * curr) . next () . replace (elem_ptr) ; (* next) . prev () . set (elem_ptr) ; (* elem_ptr) . prev () . set (* curr) ; (* elem_ptr) . next () . set (next) ; } AddToSllResult :: NoHead | AddToSllResult :: AlreadyInSll (_) => () , } } } }
    };
}

impl_122!();