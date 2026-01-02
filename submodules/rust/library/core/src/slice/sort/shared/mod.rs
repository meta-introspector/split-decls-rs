mkuse!{use crate :: marker :: Freeze ;}
mkmod!{pivot, { 
                getname!(pivot);
                getsrc!(pivot);
                getpath!(pivot);
                get_deps!(pivot);
                get_crates!(pivot);
                mkinclude!(pivot);
                 
            }}
mkmod!{smallsort, { 
                getname!(smallsort);
                getsrc!(smallsort);
                getpath!(smallsort);
                get_deps!(smallsort);
                get_crates!(smallsort);
                mkinclude!(smallsort);
                 
            }}
mkitem!{mktrait!{# [doc = " SAFETY: this is safety relevant, how does this interact with the soundness holes in"] # [doc = " specialization?"] # [rustc_unsafe_specialization_marker] pub (crate) trait FreezeMarker { }}}
mkitem!{mkimpl!{impl < T : Freeze > FreezeMarker for T { }}}

macro_rules! find_existing_run_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_existing_run in module {}", module_path!());
    };
}

mkfn!{
    find_existing_run_introspect!();
    # [doc = " Finds a run of sorted elements starting at the beginning of the slice."] # [doc = ""] # [doc = " Returns the length of the run, and a bool that is false when the run"] # [doc = " is ascending, and true if the run strictly descending."] # [inline (always)] pub (crate) fn find_existing_run < T , F : FnMut (& T , & T) -> bool > (v : & [T] , is_less : & mut F ,) -> (usize , bool) { let len = v . len () ; if len < 2 { return (len , false) ; } unsafe { let mut run_len = 2 ; let strictly_descending = is_less (v . get_unchecked (1) , v . get_unchecked (0)) ; if strictly_descending { while run_len < len && is_less (v . get_unchecked (run_len) , v . get_unchecked (run_len - 1)) { run_len += 1 ; } } else { while run_len < len && ! is_less (v . get_unchecked (run_len) , v . get_unchecked (run_len - 1)) { run_len += 1 ; } } (run_len , strictly_descending) } }
}