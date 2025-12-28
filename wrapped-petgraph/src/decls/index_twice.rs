macro_rules! deps {
    () => {
        Pair!();
    };
}

macro_rules! index_twice {
    () => {
        deps!();
        # [doc = " Get mutable references at index `a` and `b`."] fn index_twice < T > (slc : & mut [T] , a : usize , b : usize) -> Pair < & mut T > { if max (a , b) >= slc . len () { Pair :: None } else if a == b { Pair :: One (& mut slc [max (a , b)]) } else { unsafe { let ptr = slc . as_mut_ptr () ; let ar = & mut * ptr . add (a) ; let br = & mut * ptr . add (b) ; Pair :: Both (ar , br) } } }
    };
}

index_twice!();