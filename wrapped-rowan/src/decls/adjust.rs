macro_rules! deps {
    () => {
        Elem!();
        Delta!();
    };
}

macro_rules! adjust {
    () => {
        deps!();
        pub (crate) fn adjust < E : Elem > (elem : & E , from : u32 , by : Delta < u32 >) { let elem_ptr : * const E = elem ; unsafe { let mut curr = elem_ptr ; loop { let mut key = (* curr) . key () . get () ; if key >= from { key += by ; (* curr) . key () . set (key) ; } curr = (* curr) . next () . get () ; if curr == elem_ptr { break ; } } } }
    };
}

adjust!();