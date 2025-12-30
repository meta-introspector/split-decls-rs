// Generated macro for ALLOCATOR_METHODS (static)
macro_rules! Depcrate_expand_allocatorALLOCATOR_METHODS {
() => {
// Module: crate::expand::allocator
// Provides: {"ALLOCATOR_METHODS"}
// Dependencies: {}
pub static ALLOCATOR_METHODS : & [AllocatorMethod] = & [AllocatorMethod { name : sym :: alloc , inputs : & [AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout }] , output : AllocatorTy :: ResultPtr , } , AllocatorMethod { name : sym :: dealloc , inputs : & [AllocatorMethodInput { name : "ptr" , ty : AllocatorTy :: Ptr } , AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout } ,] , output : AllocatorTy :: Unit , } , AllocatorMethod { name : sym :: realloc , inputs : & [AllocatorMethodInput { name : "ptr" , ty : AllocatorTy :: Ptr } , AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout } , AllocatorMethodInput { name : "new_size" , ty : AllocatorTy :: Usize } ,] , output : AllocatorTy :: ResultPtr , } , AllocatorMethod { name : sym :: alloc_zeroed , inputs : & [AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout }] , output : AllocatorTy :: ResultPtr , } ,] ;
};
}
