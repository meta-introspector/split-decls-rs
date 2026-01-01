// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/int_overflow.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=main | COMPLEXITY=8 | LINES=23 */
// Compiler:
//
// Run-time:
//   stdout: Success
//   status: signal

fn main() {
    std::panic::set_hook(Box::new(|_| {
        println!("Success");
        std::process::abort();
    }));

    let arg_count = std::env::args().count();
    let int = isize::MAX;
    let _int = int + arg_count as isize; // overflow

    // If overflow checking is disabled, we should reach here.
    #[cfg(not(debug_assertions))]
    unsafe {
        println!("Success");
        std::process::abort();
    }
}