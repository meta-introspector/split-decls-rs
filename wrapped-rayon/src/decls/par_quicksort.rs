macro_rules! par_quicksort {
    () => {
        # [doc = " Sorts `v` using pattern-defeating quicksort in parallel."] # [doc = ""] # [doc = " The algorithm is unstable, in-place, and *O*(*n* \\* log(*n*)) worst-case."] pub (super) fn par_quicksort < T , F > (v : & mut [T] , is_less : F) where T : Send , F : Fn (& T , & T) -> bool + Sync , { if size_of :: < T > () == 0 { return ; } let limit = usize :: BITS - v . len () . leading_zeros () ; recurse (v , & is_less , None , limit) ; }
    };
}

par_quicksort!()