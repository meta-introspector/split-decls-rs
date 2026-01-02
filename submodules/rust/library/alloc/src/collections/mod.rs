mkmod!{binary_heap, { 
                getname!(binary_heap);
                getsrc!(binary_heap);
                getpath!(binary_heap);
                get_deps!(binary_heap);
                get_crates!(binary_heap);
                mkinclude!(binary_heap);
                 
            }}
mkmod!{btree, { 
                getname!(btree);
                getsrc!(btree);
                getpath!(btree);
                get_deps!(btree);
                get_crates!(btree);
                mkinclude!(btree);
                 
            }}
mkmod!{linked_list, { 
                getname!(linked_list);
                getsrc!(linked_list);
                getpath!(linked_list);
                get_deps!(linked_list);
                get_crates!(linked_list);
                mkinclude!(linked_list);
                 
            }}
mkmod!{vec_deque, { 
                getname!(vec_deque);
                getsrc!(vec_deque);
                getpath!(vec_deque);
                get_deps!(vec_deque);
                get_crates!(vec_deque);
                mkinclude!(vec_deque);
                 
            }}
mkmod!{btree_map, { 
                getname!(btree_map);
                getsrc!(btree_map);
                getpath!(btree_map);
                get_deps!(btree_map);
                get_crates!(btree_map);
                mkinclude!(btree_map);
                mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use super :: btree :: map :: * ;} 
            }}
mkmod!{btree_set, { 
                getname!(btree_set);
                getsrc!(btree_set);
                getpath!(btree_set);
                get_deps!(btree_set);
                get_crates!(btree_set);
                mkinclude!(btree_set);
                mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [cfg (not (test))] pub use super :: btree :: set :: * ;} 
            }}
mkuse!{# [cfg (not (test))] use core :: fmt :: Display ;}
mkuse!{# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] # [cfg (not (test))] pub use binary_heap :: BinaryHeap ;}
mkuse!{# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] # [cfg (not (test))] pub use btree_map :: BTreeMap ;}
mkuse!{# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] # [cfg (not (test))] pub use btree_set :: BTreeSet ;}
mkuse!{# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] # [cfg (not (test))] pub use linked_list :: LinkedList ;}
mkuse!{# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] # [cfg (not (test))] pub use vec_deque :: VecDeque ;}
mkuse!{# [cfg (not (test))] use crate :: alloc :: { Layout , LayoutError } ;}
mkitem!{mkstruct!{# [doc = " The error type for `try_reserve` methods."] # [derive (Clone , PartialEq , Eq , Debug)] # [stable (feature = "try_reserve" , since = "1.57.0")] # [cfg (not (test))] pub struct TryReserveError { kind : TryReserveErrorKind , }}}
mkuse!{# [cfg (test)] pub use realalloc :: collections :: TryReserveError ;}
mkitem!{mkimpl!{# [cfg (not (test))] impl TryReserveError { # [doc = " Details about the allocation that caused the error"] # [inline] # [must_use] # [unstable (feature = "try_reserve_kind" , reason = "Uncertain how much info should be exposed" , issue = "48043")] pub fn kind (& self) -> TryReserveErrorKind { self . kind . clone () } }}}
mkitem!{mkenum!{# [doc = " Details of the allocation that caused a `TryReserveError`"] # [derive (Clone , PartialEq , Eq , Debug)] # [unstable (feature = "try_reserve_kind" , reason = "Uncertain how much info should be exposed" , issue = "48043")] # [cfg (not (test))] pub enum TryReserveErrorKind { # [doc = " Error due to the computed capacity exceeding the collection's maximum"] # [doc = " (usually `isize::MAX` bytes)."] CapacityOverflow , # [doc = " The memory allocator returned an error"] AllocError { # [doc = " The layout of allocation request that failed"] layout : Layout , # [doc (hidden)] # [unstable (feature = "container_error_extra" , issue = "none" , reason = "\
            Enable exposing the allocator’s custom error value \
            if an associated type is added in the future: \
            https://github.com/rust-lang/wg-allocators/issues/23")] non_exhaustive : () , } , }}}
mkuse!{# [cfg (test)] pub use realalloc :: collections :: TryReserveErrorKind ;}
mkitem!{# [unstable (feature = "try_reserve_kind" , reason = "Uncertain how much info should be exposed" , issue = "48043")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] # [cfg (not (test))] impl const From < TryReserveErrorKind > for TryReserveError { # [inline] fn from (kind : TryReserveErrorKind) -> Self { Self { kind } } }}
mkitem!{# [unstable (feature = "try_reserve_kind" , reason = "new API" , issue = "48043")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] # [cfg (not (test))] impl const From < LayoutError > for TryReserveErrorKind { # [doc = " Always evaluates to [`TryReserveErrorKind::CapacityOverflow`]."] # [inline] fn from (_ : LayoutError) -> Self { TryReserveErrorKind :: CapacityOverflow } }}
mkitem!{mkimpl!{# [stable (feature = "try_reserve" , since = "1.57.0")] # [cfg (not (test))] impl Display for TryReserveError { fn fmt (& self , fmt : & mut core :: fmt :: Formatter < '_ > ,) -> core :: result :: Result < () , core :: fmt :: Error > { fmt . write_str ("memory allocation failed") ? ; let reason = match self . kind { TryReserveErrorKind :: CapacityOverflow => { " because the computed capacity exceeded the collection's maximum" } TryReserveErrorKind :: AllocError { .. } => { " because the memory allocator returned an error" } } ; fmt . write_str (reason) } }}}
mkitem!{mktrait!{# [doc = " An intermediate trait for specialization of `Extend`."] # [doc (hidden)] # [cfg (not (no_global_oom_handling))] trait SpecExtend < I : IntoIterator > { # [doc = " Extends `self` with the contents of the given iterator."] fn spec_extend (& mut self , iter : I) ; }}}
mkitem!{mkimpl!{# [stable (feature = "try_reserve" , since = "1.57.0")] # [cfg (not (test))] impl core :: error :: Error for TryReserveError { }}}