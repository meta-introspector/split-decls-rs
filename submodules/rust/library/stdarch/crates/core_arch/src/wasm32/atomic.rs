mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{unsafe extern "unadjusted" { # [link_name = "llvm.wasm.memory.atomic.wait32"] fn llvm_atomic_wait_i32 (ptr : * mut i32 , exp : i32 , timeout : i64) -> i32 ; # [link_name = "llvm.wasm.memory.atomic.wait64"] fn llvm_atomic_wait_i64 (ptr : * mut i64 , exp : i64 , timeout : i64) -> i32 ; # [link_name = "llvm.wasm.memory.atomic.notify"] fn llvm_atomic_notify (ptr : * mut i32 , cnt : i32) -> i32 ; }}

macro_rules! memory_atomic_wait32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function memory_atomic_wait32 in module {}", module_path!());
    };
}

mkfn!{
    memory_atomic_wait32_introspect!();
    # [doc = " Corresponding intrinsic to wasm's [`memory.atomic.wait32` instruction][instr]"] # [doc = ""] # [doc = " This function, when called, will block the current thread if the memory"] # [doc = " pointed to by `ptr` is equal to `expression` (performing this action"] # [doc = " atomically)."] # [doc = ""] # [doc = " The argument `timeout_ns` is a maximum number of nanoseconds the calling"] # [doc = " thread will be blocked for, if it blocks. If the timeout is negative then"] # [doc = " the calling thread will be blocked forever."] # [doc = ""] # [doc = " The calling thread can only be woken up with a call to the `wake` intrinsic"] # [doc = " once it has been blocked. Changing the memory behind `ptr` will not wake"] # [doc = " the thread once it's blocked."] # [doc = ""] # [doc = " # Return value"] # [doc = ""] # [doc = " * 0 - indicates that the thread blocked and then was woken up"] # [doc = " * 1 - the loaded value from `ptr` didn't match `expression`, the thread"] # [doc = "   didn't block"] # [doc = " * 2 - the thread blocked, but the timeout expired."] # [doc = ""] # [doc = " [instr]: https://webassembly.github.io/threads/core/syntax/instructions.html#syntax-instr-atomic-memory"] # [inline] # [cfg_attr (test , assert_instr (memory . atomic . wait32))] # [target_feature (enable = "atomics")] # [doc (alias ("memory.atomic.wait32"))] # [unstable (feature = "stdarch_wasm_atomic_wait" , issue = "77839")] pub unsafe fn memory_atomic_wait32 (ptr : * mut i32 , expression : i32 , timeout_ns : i64) -> i32 { llvm_atomic_wait_i32 (ptr , expression , timeout_ns) }
}

macro_rules! memory_atomic_wait64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function memory_atomic_wait64 in module {}", module_path!());
    };
}

mkfn!{
    memory_atomic_wait64_introspect!();
    # [doc = " Corresponding intrinsic to wasm's [`memory.atomic.wait64` instruction][instr]"] # [doc = ""] # [doc = " This function, when called, will block the current thread if the memory"] # [doc = " pointed to by `ptr` is equal to `expression` (performing this action"] # [doc = " atomically)."] # [doc = ""] # [doc = " The argument `timeout_ns` is a maximum number of nanoseconds the calling"] # [doc = " thread will be blocked for, if it blocks. If the timeout is negative then"] # [doc = " the calling thread will be blocked forever."] # [doc = ""] # [doc = " The calling thread can only be woken up with a call to the `wake` intrinsic"] # [doc = " once it has been blocked. Changing the memory behind `ptr` will not wake"] # [doc = " the thread once it's blocked."] # [doc = ""] # [doc = " # Return value"] # [doc = ""] # [doc = " * 0 - indicates that the thread blocked and then was woken up"] # [doc = " * 1 - the loaded value from `ptr` didn't match `expression`, the thread"] # [doc = "   didn't block"] # [doc = " * 2 - the thread blocked, but the timeout expired."] # [doc = ""] # [doc = " [instr]: https://webassembly.github.io/threads/core/syntax/instructions.html#syntax-instr-atomic-memory"] # [inline] # [cfg_attr (test , assert_instr (memory . atomic . wait64))] # [target_feature (enable = "atomics")] # [doc (alias ("memory.atomic.wait64"))] # [unstable (feature = "stdarch_wasm_atomic_wait" , issue = "77839")] pub unsafe fn memory_atomic_wait64 (ptr : * mut i64 , expression : i64 , timeout_ns : i64) -> i32 { llvm_atomic_wait_i64 (ptr , expression , timeout_ns) }
}

macro_rules! memory_atomic_notify_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function memory_atomic_notify in module {}", module_path!());
    };
}

mkfn!{
    memory_atomic_notify_introspect!();
    # [doc = " Corresponding intrinsic to wasm's [`memory.atomic.notify` instruction][instr]"] # [doc = ""] # [doc = " This function will notify a number of threads blocked on the address"] # [doc = " indicated by `ptr`. Threads previously blocked with the `i32_atomic_wait`"] # [doc = " and `i64_atomic_wait` functions above will be woken up."] # [doc = ""] # [doc = " The `waiters` argument indicates how many waiters should be woken up (a"] # [doc = " maximum). If the value is zero no waiters are woken up."] # [doc = ""] # [doc = " # Return value"] # [doc = ""] # [doc = " Returns the number of waiters which were actually notified."] # [doc = ""] # [doc = " [instr]: https://webassembly.github.io/threads/core/syntax/instructions.html#syntax-instr-atomic-memory"] # [inline] # [cfg_attr (test , assert_instr (memory . atomic . notify))] # [target_feature (enable = "atomics")] # [doc (alias ("memory.atomic.notify"))] # [unstable (feature = "stdarch_wasm_atomic_wait" , issue = "77839")] pub unsafe fn memory_atomic_notify (ptr : * mut i32 , waiters : u32) -> u32 { llvm_atomic_notify (ptr , waiters as i32) as u32 }
}