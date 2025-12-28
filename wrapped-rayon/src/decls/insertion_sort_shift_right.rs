macro_rules! insertion_sort_shift_right {
    () => {
        # [doc = " Sort `v` assuming `v[offset..]` is already sorted."] # [doc = ""] # [doc = " Never inline this function to avoid code bloat. It still optimizes nicely and has practically no"] # [doc = " performance impact. Even improving performance in some cases."] # [inline (never)] fn insertion_sort_shift_right < T , F > (v : & mut [T] , offset : usize , is_less : & F) where F : Fn (& T , & T) -> bool , { let len = v . len () ; assert ! (offset != 0 && offset <= len && len >= 2) ; for i in (0 .. offset) . rev () { unsafe { insert_head (& mut v [i .. len] , is_less) ; } } }
    };
}

insertion_sort_shift_right!();