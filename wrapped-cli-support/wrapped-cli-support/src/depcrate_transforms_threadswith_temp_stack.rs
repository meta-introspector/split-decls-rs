// Generated macro for with_temp_stack (function)
macro_rules! Depcrate_transforms_threadswith_temp_stack {
() => {
// Module: crate::transforms::threads
// Provides: {"with_temp_stack"}
// Dependencies: {}
# [doc = " Wraps the instructions fed by `block()` so that they can assume that the temporary, scratch"] # [doc = " stack is usable. Clobbers `stack.pointer`."] fn with_temp_stack (body : & mut InstrSeqBuilder < '_ > , memory : MemoryId , stack : & Stack , block : impl Fn (& mut InstrSeqBuilder < '_ >) ,) { use walrus :: ir :: * ; body . i32_const (stack . temp) . global_set (stack . pointer) ; body . loop_ (None , | loop_ | { let loop_id = loop_ . id () ; loop_ . i32_const (stack . temp_lock) . i32_const (0) . i32_const (1) . cmpxchg (memory , AtomicWidth :: I32 , ATOMIC_MEM_ARG) . if_else (None , | body | { body . i32_const (stack . temp_lock) . i32_const (1) . i64_const (- 1) . atomic_wait (memory , ATOMIC_MEM_ARG , false) . drop () . br (loop_id) ; } , | _ | { } ,) ; }) ; block (body) ; body . i32_const (stack . temp_lock) . i32_const (0) . store (memory , StoreKind :: I32 { atomic : true } , ATOMIC_MEM_ARG) . i32_const (stack . temp_lock) . i32_const (1) . atomic_notify (memory , ATOMIC_MEM_ARG) . drop () ; }
};
}
