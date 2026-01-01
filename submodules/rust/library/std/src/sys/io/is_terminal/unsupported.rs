// SRC: ../rust/library/std/src/sys/io/is_terminal/unsupported.rs
pub fn is_terminal<T>(_: &T) -> bool {
    false
}
