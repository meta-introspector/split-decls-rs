macro_rules! insertion_sort_shift_left {
    () => {
        # [doc = " Sort `v` assuming `v[..offset]` is already sorted."] # [doc = ""] # [doc = " Never inline this function to avoid code bloat. It still optimizes nicely and has practically no"] # [doc = " performance impact. Even improving performance in some cases."] # [inline (never)] fn insertion_sort_shift_left < T , F > (v : & mut [T] , offset : usize , is_less : & F) where F : Fn (& T , & T) -> bool , { let len = v . len () ; assert ! (offset != 0 && offset <= len) ; for i in offset .. len { unsafe { insert_tail (& mut v [..= i] , is_less) ; } } }
    };
}

insertion_sort_shift_left!()