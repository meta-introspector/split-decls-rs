// Generated macro for allocate_task (macro)
macro_rules! Depcrate_rawallocate_task {
() => {
// Module: crate::raw
// Provides: {"allocate_task"}
// Dependencies: {}
# [doc = " Allocates a task with the given `future` and `schedule` function."] # [doc = ""] # [doc = " It is assumed that initially only the `Runnable` and the `Task` exist."] # [doc = ""] # [doc = " Use a macro to brute force inlining to minimize stack copies of potentially"] # [doc = " large futures."] macro_rules ! allocate_task { ($ f : tt , $ s : tt , $ m : tt , $ builder : ident , $ schedule : ident , $ raw : ident => $ future : block) => { { let allocation = alloc :: alloc :: alloc (RawTask ::<$ f , <$ f as Future >:: Output , $ s , $ m >:: TASK_LAYOUT . layout) ; let ptr = NonNull :: new (allocation as * mut ()) . unwrap_or_else (|| crate :: utils :: abort ()) ; let $ raw = RawTask ::<$ f , <$ f as Future >:: Output , $ s , $ m >:: from_ptr (ptr . as_ptr ()) ; let crate :: Builder { metadata , # [cfg (feature = "std")] propagate_panic , } = $ builder ; ($ raw . header as * mut HeaderWithMetadata <$ m >) . write (HeaderWithMetadata { header : Header { # [cfg (not (feature = "portable-atomic"))] state : core :: sync :: atomic :: AtomicUsize :: new (SCHEDULED | TASK | REFERENCE) , # [cfg (feature = "portable-atomic")] state : portable_atomic :: AtomicUsize :: new (SCHEDULED | TASK | REFERENCE) , awaiter : core :: cell :: UnsafeCell :: new (None) , vtable : & RawTask ::<$ f , <$ f as Future >:: Output , $ s , $ m >:: TASK_VTABLE , # [cfg (feature = "std")] propagate_panic , } , metadata , }) ; ($ raw . schedule as * mut S) . write ($ schedule) ; let bomb = crate :: utils :: Bomb ; $ raw . future . write ($ future) ; mem :: forget (bomb) ; ptr } } ; }
};
}
