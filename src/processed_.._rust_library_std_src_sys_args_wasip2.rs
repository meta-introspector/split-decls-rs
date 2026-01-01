// SRC: ../rust/library/std/src/sys/args/wasip2.rs
pub use super::common::Args;

/// Returns the command line arguments
pub fn args() -> Args {
    Args::new(wasip2::cli::environment::get_arguments().into_iter().map(|arg| arg.into()).collect())
}
