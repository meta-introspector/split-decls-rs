// SRC: ../rust/library/std/src/sys/random/wasip1.rs
pub fn fill_bytes(bytes: &mut [u8]) {
    unsafe {
        wasi::random_get(bytes.as_mut_ptr(), bytes.len()).expect("failed to generate random data")
    }
}
