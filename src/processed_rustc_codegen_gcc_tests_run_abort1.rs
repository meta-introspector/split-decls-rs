// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/abort1.rs
// Compiler:
//
// Run-time:
//   status: signal

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

fn test_fail() -> ! {
    unsafe { intrinsics::abort() };
}

#[unsafe(no_mangle)]
extern "C" fn main(argc: i32, _argv: *const *const u8) -> i32 {
    test_fail();
}