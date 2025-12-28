macro_rules! deps {
    () => {
        InsertionHole!();
    };
}

macro_rules! insert_tail {
    () => {
        deps!();
        # [doc = " Inserts `v[v.len() - 1]` into pre-sorted sequence `v[..v.len() - 1]` so that whole `v[..]`"] # [doc = " becomes sorted."] unsafe fn insert_tail < T , F > (v : & mut [T] , is_less : & F) where F : Fn (& T , & T) -> bool , { debug_assert ! (v . len () >= 2) ; let arr_ptr = v . as_mut_ptr () ; let i = v . len () - 1 ; unsafe { let i_ptr = arr_ptr . add (i) ; if is_less (& * i_ptr , & * i_ptr . sub (1)) { let tmp = mem :: ManuallyDrop :: new (ptr :: read (i_ptr)) ; let mut hole = InsertionHole { src : & * tmp , dest : i_ptr . sub (1) , } ; ptr :: copy_nonoverlapping (hole . dest , i_ptr , 1) ; for j in (0 .. (i - 1)) . rev () { let j_ptr = arr_ptr . add (j) ; if ! is_less (& * tmp , & * j_ptr) { break ; } ptr :: copy_nonoverlapping (j_ptr , hole . dest , 1) ; hole . dest = j_ptr ; } } } }
    };
}

insert_tail!();