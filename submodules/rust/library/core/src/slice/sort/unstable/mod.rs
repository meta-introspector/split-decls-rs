mkuse!{use crate :: mem :: SizedTypeProperties ;}
mkuse!{# [cfg (not (any (feature = "optimize_for_size" , target_pointer_width = "16")))] use crate :: slice :: sort :: shared :: find_existing_run ;}
mkuse!{# [cfg (not (any (feature = "optimize_for_size" , target_pointer_width = "16")))] use crate :: slice :: sort :: shared :: smallsort :: insertion_sort_shift_left ;}
mkuse!{use crate :: { cfg_select , intrinsics } ;}
mkmod!{heapsort, { 
                getname!(heapsort);
                getsrc!(heapsort);
                getpath!(heapsort);
                get_deps!(heapsort);
                get_crates!(heapsort);
                mkinclude!(heapsort);
                 
            }}
mkmod!{quicksort, { 
                getname!(quicksort);
                getsrc!(quicksort);
                getpath!(quicksort);
                get_deps!(quicksort);
                get_crates!(quicksort);
                mkinclude!(quicksort);
                 
            }}

macro_rules! sort_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sort in module {}", module_path!());
    };
}

mkfn!{
    sort_introspect!();
    # [doc = " Unstable sort called ipnsort by Lukas Bergdoll and Orson Peters."] # [doc = " Design document:"] # [doc = " <https://github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_introduction/text.md>"] # [doc = ""] # [doc = " Upholds all safety properties outlined here:"] # [doc = " <https://github.com/Voultapher/sort-research-rs/blob/main/writeup/sort_safety/text.md>"] # [inline (always)] pub fn sort < T , F : FnMut (& T , & T) -> bool > (v : & mut [T] , is_less : & mut F) { if T :: IS_ZST { return ; } let len = v . len () ; if intrinsics :: likely (len < 2) { return ; } cfg_select ! { any (feature = "optimize_for_size" , target_pointer_width = "16") => { heapsort :: heapsort (v , is_less) ; } _ => { const MAX_LEN_ALWAYS_INSERTION_SORT : usize = 20 ; if intrinsics :: likely (len <= MAX_LEN_ALWAYS_INSERTION_SORT) { insertion_sort_shift_left (v , 1 , is_less) ; return ; } ipnsort (v , is_less) ; } } }
}

macro_rules! ipnsort_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ipnsort in module {}", module_path!());
    };
}

mkfn!{
    ipnsort_introspect!();
    # [doc = " See [`sort`]"] # [doc = ""] # [doc = " Deliberately don't inline the main sorting routine entrypoint to ensure the"] # [doc = " inlined insertion sort i-cache footprint remains minimal."] # [cfg (not (any (feature = "optimize_for_size" , target_pointer_width = "16")))] # [inline (never)] fn ipnsort < T , F > (v : & mut [T] , is_less : & mut F) where F : FnMut (& T , & T) -> bool , { let len = v . len () ; let (run_len , was_reversed) = find_existing_run (v , is_less) ; unsafe { intrinsics :: assume (run_len <= len) } ; if run_len == len { if was_reversed { v . reverse () ; } return ; } let limit = 2 * (len | 1) . ilog2 () ; crate :: slice :: sort :: unstable :: quicksort :: quicksort (v , None , limit , is_less) ; }
}