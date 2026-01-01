// SRC: ../rust/compiler/rustc_codegen_gcc/example/alloc_example.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=puts | COMPLEXITY=2 | LINES=17 */
#[feature(core_intrinsics, alloc_error_handler, lang_items)]
#[no_std]
#[no_main]
#[allow(internal_features)]


use alloc::boxed::Box;

use alloc_system::System;

#[global_allocator]
static ALLOC: System = System;

#[link(name = "c")]
extern "C" {
    fn puts(s: *const u8) -> i32;
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=panic_handler | COMPLEXITY=2 | LINES=5 */

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    core::intrinsics::abort();
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=alloc_error_handler | COMPLEXITY=2 | LINES=5 */

#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    core::intrinsics::abort();
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=eh_personality | COMPLEXITY=5 | LINES=5 */

#[lang = "eh_personality"]
fn eh_personality() -> ! {
    loop {}
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=5 */

#[unsafe(no_mangle)]
unsafe extern "C" fn _Unwind_Resume() {
    core::intrinsics::unreachable();
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=10 */

#[unsafe(no_mangle)]
extern "C" fn main(_argc: core::ffi::c_int, _argv: *const *const u8) -> core::ffi::c_int {
    let world: Box<&str> = Box::new("Hello World!\0");
    unsafe {
        puts(*world as *const str as *const u8);
    }

    0
}