mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: marker :: { Send , Sized , Sync , Unpin } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: ops :: { Drop , Fn , FnMut , FnOnce } ;}
mkuse!{# [stable (feature = "async_closure" , since = "1.85.0")] # [doc (no_inline)] pub use crate :: ops :: { AsyncFn , AsyncFnMut , AsyncFnOnce } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: mem :: drop ;}
mkuse!{# [stable (feature = "size_of_prelude" , since = "1.80.0")] # [doc (no_inline)] pub use crate :: mem :: { align_of , align_of_val , size_of , size_of_val } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: convert :: { AsMut , AsRef , From , Into } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: iter :: { DoubleEndedIterator , ExactSizeIterator } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: iter :: { Extend , IntoIterator , Iterator } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: option :: Option :: { self , None , Some } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: result :: Result :: { self , Err , Ok } ;}
mkuse!{# [stable (feature = "builtin_macro_prelude" , since = "1.38.0")] # [doc (no_inline)] pub use core :: prelude :: v1 :: { assert , cfg , column , compile_error , concat , env , file , format_args , format_args_nl , include , include_bytes , include_str , line , log_syntax , module_path , option_env , stringify , trace_macros , Clone , Copy , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd , } ;}
mkuse!{# [unstable (feature = "concat_bytes" , issue = "87555" , reason = "`concat_bytes` is not stable enough for use and is subject to change")] # [doc (no_inline)] pub use core :: prelude :: v1 :: concat_bytes ;}
mkuse!{# [stable (feature = "builtin_macro_prelude" , since = "1.38.0")] pub use core :: prelude :: v1 :: { alloc_error_handler , bench , derive , global_allocator , test , test_case , } ;}
mkuse!{# [unstable (feature = "derive_const" , issue = "118304")] pub use core :: prelude :: v1 :: derive_const ;}
mkuse!{# [unstable (feature = "cfg_accessible" , issue = "64797" , reason = "`cfg_accessible` is not fully implemented")] pub use core :: prelude :: v1 :: cfg_accessible ;}
mkuse!{# [unstable (feature = "cfg_eval" , issue = "82679" , reason = "`cfg_eval` is a recently implemented feature")] pub use core :: prelude :: v1 :: cfg_eval ;}
mkuse!{# [unstable (feature = "type_ascription" , issue = "23416" , reason = "placeholder syntax for type ascription")] pub use core :: prelude :: v1 :: type_ascribe ;}
mkuse!{# [unstable (feature = "deref_patterns" , issue = "87121" , reason = "placeholder syntax for deref patterns")] pub use core :: prelude :: v1 :: deref ;}
mkuse!{# [unstable (feature = "type_alias_impl_trait" , issue = "63063" , reason = "`type_alias_impl_trait` has open design concerns")] pub use core :: prelude :: v1 :: define_opaque ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: borrow :: ToOwned ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: boxed :: Box ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: string :: { String , ToString } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use crate :: vec :: Vec ;}