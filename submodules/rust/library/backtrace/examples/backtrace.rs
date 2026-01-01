// SRC: ../rust/library/backtrace/examples/backtrace.rs
use backtrace::Backtrace;

fn main() {
    println!("{:?}", Backtrace::new());
}
