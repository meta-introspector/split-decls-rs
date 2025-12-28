macro_rules! deps {
    () => {
        InsertionHole!();
    };
}

macro_rules! insert_head {
    () => {
        deps!();
        # [doc = " Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted."] # [doc = ""] # [doc = " This is the integral subroutine of insertion sort."] unsafe fn insert_head < T , F > (v : & mut [T] , is_less : & F) where F : Fn (& T , & T) -> bool , { debug_assert ! (v . len () >= 2) ; unsafe { if is_less (v . get_unchecked (1) , v . get_unchecked (0)) { let arr_ptr = v . as_mut_ptr () ; let tmp = mem :: ManuallyDrop :: new (ptr :: read (arr_ptr)) ; let mut hole = InsertionHole { src : & * tmp , dest : arr_ptr . add (1) , } ; ptr :: copy_nonoverlapping (arr_ptr . add (1) , arr_ptr . add (0) , 1) ; for i in 2 .. v . len () { if ! is_less (v . get_unchecked (i) , & * tmp) { break ; } ptr :: copy_nonoverlapping (arr_ptr . add (i) , arr_ptr . add (i - 1) , 1) ; hole . dest = arr_ptr . add (i) ; } } } }
    };
}

insert_head!()