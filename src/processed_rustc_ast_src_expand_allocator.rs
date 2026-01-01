/* FP:allocator.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_USE_0001
/* FP:allocator.rs-0002 */ use rustc_macros :: HashStable_Generic ;
/* FP:allocator.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_USE_0002
/* FP:allocator.rs-0004 */ use crate :: rustc_complete :: { Symbol , sym } ;
/* FP:allocator.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_ENUM_0003
/* FP:allocator.rs-0006 */ # [derive (Clone , Debug , Copy , Eq , PartialEq , HashStable_Generic)] pub enum AllocatorKind { Global , Default , }
/* FP:allocator.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_FN_0004
/* FP:allocator.rs-0008 */ pub fn global_fn_name (base : Symbol) -> String { format ! ("__rust_{base}") }
/* FP:allocator.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_FN_0005
/* FP:allocator.rs-0010 */ pub fn default_fn_name (base : Symbol) -> String { format ! ("__rdl_{base}") }
/* FP:allocator.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_FN_0006
/* FP:allocator.rs-0012 */ pub fn alloc_error_handler_name (alloc_error_handler_kind : AllocatorKind) -> & 'static str { match alloc_error_handler_kind { AllocatorKind :: Global => "__rg_oom" , AllocatorKind :: Default => "__rdl_oom" , } }
/* FP:allocator.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_CONST_0007
/* FP:allocator.rs-0014 */ pub const NO_ALLOC_SHIM_IS_UNSTABLE : & str = "__rust_no_alloc_shim_is_unstable_v2" ;
/* FP:allocator.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_ENUM_0008
/* FP:allocator.rs-0016 */ pub enum AllocatorTy { Layout , Ptr , ResultPtr , Unit , Usize , }
/* FP:allocator.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_STRUCT_0009
/* FP:allocator.rs-0018 */ pub struct AllocatorMethod { pub name : Symbol , pub inputs : & 'static [AllocatorMethodInput] , pub output : AllocatorTy , }
/* FP:allocator.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_STRUCT_0010
/* FP:allocator.rs-0020 */ pub struct AllocatorMethodInput { pub name : & 'static str , pub ty : AllocatorTy , }
/* FP:allocator.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_expand_allocator_STATIC_0011
/* FP:allocator.rs-0022 */ pub static ALLOCATOR_METHODS : & [AllocatorMethod] = & [AllocatorMethod { name : sym :: alloc , inputs : & [AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout }] , output : AllocatorTy :: ResultPtr , } , AllocatorMethod { name : sym :: dealloc , inputs : & [AllocatorMethodInput { name : "ptr" , ty : AllocatorTy :: Ptr } , AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout } ,] , output : AllocatorTy :: Unit , } , AllocatorMethod { name : sym :: realloc , inputs : & [AllocatorMethodInput { name : "ptr" , ty : AllocatorTy :: Ptr } , AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout } , AllocatorMethodInput { name : "new_size" , ty : AllocatorTy :: Usize } ,] , output : AllocatorTy :: ResultPtr , } , AllocatorMethod { name : sym :: alloc_zeroed , inputs : & [AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout }] , output : AllocatorTy :: ResultPtr , } ,] ;