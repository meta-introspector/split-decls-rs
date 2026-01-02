mkuse!{# [stable (feature = "try_reserve" , since = "1.57.0")] pub use alloc_crate :: collections :: TryReserveError ;}
mkuse!{# [unstable (feature = "try_reserve_kind" , reason = "Uncertain how much info should be exposed" , issue = "48043")] pub use alloc_crate :: collections :: TryReserveErrorKind ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: collections :: { BTreeMap , BTreeSet , BinaryHeap } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: collections :: { LinkedList , VecDeque } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: collections :: { binary_heap , btree_map , btree_set } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: collections :: { linked_list , vec_deque } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (inline)] pub use self :: hash_map :: HashMap ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (inline)] pub use self :: hash_set :: HashSet ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [deprecated (note = "moved to `std::ops::Bound`" , since = "1.26.0")] # [doc (hidden)] pub use crate :: ops :: Bound ;}
mkmod!{hash, { 
                getname!(hash);
                getsrc!(hash);
                getpath!(hash);
                get_deps!(hash);
                get_crates!(hash);
                mkinclude!(hash);
                 
            }}
mkmod!{hash_map, { 
                getname!(hash_map);
                getsrc!(hash_map);
                getpath!(hash_map);
                get_deps!(hash_map);
                get_crates!(hash_map);
                mkinclude!(hash_map);
                mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use super :: hash :: map :: * ;}
mkuse!{# [stable (feature = "hashmap_build_hasher" , since = "1.7.0")] pub use crate :: hash :: random :: DefaultHasher ;}
mkuse!{# [stable (feature = "hashmap_build_hasher" , since = "1.7.0")] pub use crate :: hash :: random :: RandomState ;} 
            }}
mkmod!{hash_set, { 
                getname!(hash_set);
                getsrc!(hash_set);
                getpath!(hash_set);
                get_deps!(hash_set);
                get_crates!(hash_set);
                mkinclude!(hash_set);
                mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use super :: hash :: set :: * ;} 
            }}