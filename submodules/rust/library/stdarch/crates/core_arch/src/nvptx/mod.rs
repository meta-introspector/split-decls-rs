mkuse!{use crate :: ffi :: c_void ;}
mkmod!{packed, { 
                getname!(packed);
                getsrc!(packed);
                getpath!(packed);
                get_deps!(packed);
                get_crates!(packed);
                mkinclude!(packed);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub use packed :: * ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.nvvm.barrier0"] fn syncthreads () -> () ; # [link_name = "llvm.nvvm.read.ptx.sreg.ntid.x"] fn block_dim_x () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.ntid.y"] fn block_dim_y () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.ntid.z"] fn block_dim_z () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.ctaid.x"] fn block_idx_x () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.ctaid.y"] fn block_idx_y () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.ctaid.z"] fn block_idx_z () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.nctaid.x"] fn grid_dim_x () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.nctaid.y"] fn grid_dim_y () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.nctaid.z"] fn grid_dim_z () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.tid.x"] fn thread_idx_x () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.tid.y"] fn thread_idx_y () -> i32 ; # [link_name = "llvm.nvvm.read.ptx.sreg.tid.z"] fn thread_idx_z () -> i32 ; }}

macro_rules! _syncthreads_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _syncthreads in module {}", module_path!());
    };
}

mkfn!{
    _syncthreads_introspect!();
    # [doc = " Synchronizes all threads in the block."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _syncthreads () -> () { syncthreads () }
}

macro_rules! _block_dim_x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _block_dim_x in module {}", module_path!());
    };
}

mkfn!{
    _block_dim_x_introspect!();
    # [doc = " x-th thread-block dimension."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _block_dim_x () -> i32 { block_dim_x () }
}

macro_rules! _block_dim_y_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _block_dim_y in module {}", module_path!());
    };
}

mkfn!{
    _block_dim_y_introspect!();
    # [doc = " y-th thread-block dimension."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _block_dim_y () -> i32 { block_dim_y () }
}

macro_rules! _block_dim_z_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _block_dim_z in module {}", module_path!());
    };
}

mkfn!{
    _block_dim_z_introspect!();
    # [doc = " z-th thread-block dimension."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _block_dim_z () -> i32 { block_dim_z () }
}

macro_rules! _block_idx_x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _block_idx_x in module {}", module_path!());
    };
}

mkfn!{
    _block_idx_x_introspect!();
    # [doc = " x-th thread-block index."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _block_idx_x () -> i32 { block_idx_x () }
}

macro_rules! _block_idx_y_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _block_idx_y in module {}", module_path!());
    };
}

mkfn!{
    _block_idx_y_introspect!();
    # [doc = " y-th thread-block index."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _block_idx_y () -> i32 { block_idx_y () }
}

macro_rules! _block_idx_z_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _block_idx_z in module {}", module_path!());
    };
}

mkfn!{
    _block_idx_z_introspect!();
    # [doc = " z-th thread-block index."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _block_idx_z () -> i32 { block_idx_z () }
}

macro_rules! _grid_dim_x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _grid_dim_x in module {}", module_path!());
    };
}

mkfn!{
    _grid_dim_x_introspect!();
    # [doc = " x-th block-grid dimension."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _grid_dim_x () -> i32 { grid_dim_x () }
}

macro_rules! _grid_dim_y_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _grid_dim_y in module {}", module_path!());
    };
}

mkfn!{
    _grid_dim_y_introspect!();
    # [doc = " y-th block-grid dimension."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _grid_dim_y () -> i32 { grid_dim_y () }
}

macro_rules! _grid_dim_z_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _grid_dim_z in module {}", module_path!());
    };
}

mkfn!{
    _grid_dim_z_introspect!();
    # [doc = " z-th block-grid dimension."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _grid_dim_z () -> i32 { grid_dim_z () }
}

macro_rules! _thread_idx_x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _thread_idx_x in module {}", module_path!());
    };
}

mkfn!{
    _thread_idx_x_introspect!();
    # [doc = " x-th thread index."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _thread_idx_x () -> i32 { thread_idx_x () }
}

macro_rules! _thread_idx_y_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _thread_idx_y in module {}", module_path!());
    };
}

mkfn!{
    _thread_idx_y_introspect!();
    # [doc = " y-th thread index."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _thread_idx_y () -> i32 { thread_idx_y () }
}

macro_rules! _thread_idx_z_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _thread_idx_z in module {}", module_path!());
    };
}

mkfn!{
    _thread_idx_z_introspect!();
    # [doc = " z-th thread index."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn _thread_idx_z () -> i32 { thread_idx_z () }
}

macro_rules! trap_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trap in module {}", module_path!());
    };
}

mkfn!{
    trap_introspect!();
    # [doc = " Generates the trap instruction `TRAP`"] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn trap () -> ! { crate :: intrinsics :: abort () }
}
mkitem!{unsafe extern "C" { # [doc = " Print formatted output from a kernel to a host-side output stream."] # [doc = ""] # [doc = " Syscall arguments:"] # [doc = " * `status`: The status value that is returned by `vprintf`."] # [doc = " * `format`: A pointer to the format specifier input (uses common `printf` format)."] # [doc = " * `valist`: A pointer to the valist input."] # [doc = ""] # [doc = " ```"] # [doc = " #[repr(C)]"] # [doc = " struct PrintArgs(f32, f32, f32, i32);"] # [doc = ""] # [doc = " vprintf("] # [doc = "     \"int(%f + %f) = int(%f) = %d\\n\".as_ptr(),"] # [doc = "     transmute(&PrintArgs(a, b, a + b, (a + b) as i32)),"] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " Sources:"] # [doc = " [Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#formatted-output),"] # [doc = " [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls)."] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub fn vprintf (format : * const u8 , valist : * const c_void) -> i32 ; # [doc = " Allocate memory dynamically from a fixed-size heap in global memory."] # [doc = ""] # [doc = " The CUDA in-kernel `malloc()` function allocates at least `size` bytes"] # [doc = " from the device heap and returns a pointer to the allocated memory"] # [doc = " or `NULL` if insufficient memory exists to fulfill the request."] # [doc = ""] # [doc = " The returned pointer is guaranteed to be aligned to a 16-byte boundary."] # [doc = ""] # [doc = " The memory allocated by a given CUDA thread via `malloc()` remains allocated"] # [doc = " for the lifetime of the CUDA context, or until it is explicitly released"] # [doc = " by a call to `free()`. It can be used by any other CUDA threads"] # [doc = " even from subsequent kernel launches."] # [doc = ""] # [doc = " Sources:"] # [doc = " [Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#dynamic-global-memory-allocation-and-operations),"] # [doc = " [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls)."] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub fn malloc (size : usize) -> * mut c_void ; # [doc = " Free previously dynamically allocated memory."] # [doc = ""] # [doc = " The CUDA in-kernel `free()` function deallocates the memory pointed to by `ptr`,"] # [doc = " which must have been returned by a previous call to `malloc()`. If `ptr` is NULL,"] # [doc = " the call to `free()` is ignored."] # [doc = ""] # [doc = " Any CUDA thread may free memory allocated by another thread, but care should be taken"] # [doc = " to ensure that the same pointer is not freed more than once. Repeated calls to `free()`"] # [doc = " with the same `ptr` has undefined behavior."] # [doc = ""] # [doc = " Sources:"] # [doc = " [Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#dynamic-global-memory-allocation-and-operations),"] # [doc = " [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls)."] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub fn free (ptr : * mut c_void) ; fn __assertfail (message : * const u8 , file : * const u8 , line : u32 , function : * const u8 , char_size : usize ,) ; }}

macro_rules! __assert_fail_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __assert_fail in module {}", module_path!());
    };
}

mkfn!{
    __assert_fail_introspect!();
    # [doc = " Syscall to be used whenever the *assert expression produces a `false` value*."] # [doc = ""] # [doc = " Syscall arguments:"] # [doc = " * `message`: The pointer to the string that should be output."] # [doc = " * `file`: The pointer to the file name string associated with the assert."] # [doc = " * `line`: The line number associated with the assert."] # [doc = " * `function`: The pointer to the function name string associated with the assert."] # [doc = ""] # [doc = " Source:"] # [doc = " [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls)."] # [inline] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn __assert_fail (message : * const u8 , file : * const u8 , line : u32 , function : * const u8) { __assertfail (message , file , line , function , 1) }
}