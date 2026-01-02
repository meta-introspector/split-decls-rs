mkuse!{# [cfg (not (any (feature = "optimize_for_size" , target_pointer_width = "16")))] use crate :: cmp ;}
mkuse!{use crate :: mem :: { MaybeUninit , SizedTypeProperties } ;}
mkuse!{# [cfg (not (any (feature = "optimize_for_size" , target_pointer_width = "16")))] use crate :: slice :: sort :: shared :: smallsort :: { SMALL_SORT_GENERAL_SCRATCH_LEN , StableSmallSortTypeImpl , insertion_sort_shift_left , } ;}
mkuse!{use crate :: { cfg_select , intrinsics } ;}
mkmod!{merge, { 
                getname!(merge);
                getsrc!(merge);
                getpath!(merge);
                get_deps!(merge);
                get_crates!(merge);
                mkinclude!(merge);
                 
            }}
mkmod!{drift, { 
                getname!(drift);
                getsrc!(drift);
                getpath!(drift);
                get_deps!(drift);
                get_crates!(drift);
                mkinclude!(drift);
                 
            }}
mkmod!{quicksort, { 
                getname!(quicksort);
                getsrc!(quicksort);
                getpath!(quicksort);
                get_deps!(quicksort);
                get_crates!(quicksort);
                mkinclude!(quicksort);
                 
            }}
mkmod!{tiny, { 
                getname!(tiny);
                getsrc!(tiny);
                getpath!(tiny);
                get_deps!(tiny);
                get_crates!(tiny);
                mkinclude!(tiny);
                 
            }}

macro_rules! sort_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sort in module {}", module_path!());
    };
}

mkfn!{
    sort_introspect!();
    # [doc = " Stable sort called driftsort by Orson Peters and Lukas Bergdoll."] # [doc = " Design document:"] # [doc = " <https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md>"] # [doc = ""] # [doc = " Upholds all safety properties outlined here:"] # [doc = " <https://github.com/Voultapher/sort-research-rs/blob/main/writeup/sort_safety/text.md>"] # [inline (always)] pub fn sort < T , F : FnMut (& T , & T) -> bool , BufT : BufGuard < T > > (v : & mut [T] , is_less : & mut F) { if T :: IS_ZST { return ; } let len = v . len () ; if intrinsics :: likely (len < 2) { return ; } cfg_select ! { any (feature = "optimize_for_size" , target_pointer_width = "16") => { let alloc_len = len / 2 ; cfg_select ! { target_pointer_width = "16" => { let mut heap_buf = BufT :: with_capacity (alloc_len) ; let scratch = heap_buf . as_uninit_slice_mut () ; } _ => { let mut stack_buf = AlignedStorage ::< T , 4096 >:: new () ; let stack_scratch = stack_buf . as_uninit_slice_mut () ; let mut heap_buf ; let scratch = if stack_scratch . len () >= alloc_len { stack_scratch } else { heap_buf = BufT :: with_capacity (alloc_len) ; heap_buf . as_uninit_slice_mut () } ; } } tiny :: mergesort (v , scratch , is_less) ; } _ => { const MAX_LEN_ALWAYS_INSERTION_SORT : usize = 20 ; if intrinsics :: likely (len <= MAX_LEN_ALWAYS_INSERTION_SORT) { insertion_sort_shift_left (v , 1 , is_less) ; return ; } driftsort_main ::< T , F , BufT > (v , is_less) ; } } }
}

macro_rules! driftsort_main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function driftsort_main in module {}", module_path!());
    };
}

mkfn!{
    driftsort_main_introspect!();
    # [doc = " See [`sort`]"] # [doc = ""] # [doc = " Deliberately don't inline the main sorting routine entrypoint to ensure the"] # [doc = " inlined insertion sort i-cache footprint remains minimal."] # [cfg (not (any (feature = "optimize_for_size" , target_pointer_width = "16")))] # [inline (never)] fn driftsort_main < T , F : FnMut (& T , & T) -> bool , BufT : BufGuard < T > > (v : & mut [T] , is_less : & mut F) { const MAX_FULL_ALLOC_BYTES : usize = 8_000_000 ; let max_full_alloc = MAX_FULL_ALLOC_BYTES / size_of :: < T > () ; let len = v . len () ; let alloc_len = cmp :: max (cmp :: max (len - len / 2 , cmp :: min (len , max_full_alloc)) , SMALL_SORT_GENERAL_SCRATCH_LEN ,) ; let mut stack_buf = AlignedStorage :: < T , 4096 > :: new () ; let stack_scratch = stack_buf . as_uninit_slice_mut () ; let mut heap_buf ; let scratch = if stack_scratch . len () >= alloc_len { stack_scratch } else { heap_buf = BufT :: with_capacity (alloc_len) ; heap_buf . as_uninit_slice_mut () } ; let eager_sort = len <= T :: small_sort_threshold () * 2 ; crate :: slice :: sort :: stable :: drift :: sort (v , scratch , eager_sort , is_less) ; }
}
mkitem!{mktrait!{# [doc (hidden)] # [doc = " Abstracts owned memory buffer, so that sort code can live in core where no allocation is"] # [doc = " possible. This trait can then be implemented in a place that has access to allocation."] pub trait BufGuard < T > { # [doc = " Creates new buffer that holds at least `capacity` memory."] fn with_capacity (capacity : usize) -> Self ; # [doc = " Returns mutable access to uninitialized memory owned by the buffer."] fn as_uninit_slice_mut (& mut self) -> & mut [MaybeUninit < T >] ; }}}
mkitem!{mkstruct!{# [repr (C)] struct AlignedStorage < T , const N : usize > { _align : [T ; 0] , storage : [MaybeUninit < u8 > ; N] , }}}
mkitem!{mkimpl!{impl < T , const N : usize > AlignedStorage < T , N > { fn new () -> Self { Self { _align : [] , storage : [const { MaybeUninit :: uninit () } ; N] } } fn as_uninit_slice_mut (& mut self) -> & mut [MaybeUninit < T >] { let len = N / size_of :: < T > () ; unsafe { core :: slice :: from_raw_parts_mut (self . storage . as_mut_ptr () . cast () , len) } } }}}