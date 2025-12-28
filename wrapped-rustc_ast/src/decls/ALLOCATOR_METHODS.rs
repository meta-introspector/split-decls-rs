macro_rules! deps {
    () => {
        AllocatorMethodInput!();
        AllocatorTy!();
        AllocatorMethod!();
    };
}

macro_rules! ALLOCATOR_METHODS {
    () => {
        deps!();
        pub static ALLOCATOR_METHODS : & [AllocatorMethod] = & [AllocatorMethod { name : sym :: alloc , inputs : & [AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout }] , output : AllocatorTy :: ResultPtr , } , AllocatorMethod { name : sym :: dealloc , inputs : & [AllocatorMethodInput { name : "ptr" , ty : AllocatorTy :: Ptr } , AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout } ,] , output : AllocatorTy :: Unit , } , AllocatorMethod { name : sym :: realloc , inputs : & [AllocatorMethodInput { name : "ptr" , ty : AllocatorTy :: Ptr } , AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout } , AllocatorMethodInput { name : "new_size" , ty : AllocatorTy :: Usize } ,] , output : AllocatorTy :: ResultPtr , } , AllocatorMethod { name : sym :: alloc_zeroed , inputs : & [AllocatorMethodInput { name : "layout" , ty : AllocatorTy :: Layout }] , output : AllocatorTy :: ResultPtr , } ,] ;
    };
}

ALLOCATOR_METHODS!();