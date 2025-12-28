macro_rules! partial_insertion_sort {
    () => {
        # [doc = " Partially sorts a slice by shifting several out-of-order elements around."] # [doc = ""] # [doc = " Returns `true` if the slice is sorted at the end. This function is *O*(*n*) worst-case."] # [cold] fn partial_insertion_sort < T , F > (v : & mut [T] , is_less : & F) -> bool where F : Fn (& T , & T) -> bool , { const MAX_STEPS : usize = 5 ; const SHORTEST_SHIFTING : usize = 50 ; let len = v . len () ; let mut i = 1 ; for _ in 0 .. MAX_STEPS { unsafe { while i < len && ! is_less (v . get_unchecked (i) , v . get_unchecked (i - 1)) { i += 1 ; } } if i == len { return true ; } if len < SHORTEST_SHIFTING { return false ; } v . swap (i - 1 , i) ; if i >= 2 { insertion_sort_shift_left (& mut v [.. i] , i - 1 , is_less) ; insertion_sort_shift_right (& mut v [.. i] , 1 , is_less) ; } } false }
    };
}

partial_insertion_sort!();