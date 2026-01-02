// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/random/unix_legacy.rs
// Error: expected square brackets
// Problematic line: line 15


static DEVICE: OnceLock<File> = OnceLock::new();

pub fn fill_bytes(bytes: &mut [u8]) {
    DEVICE
        .get_or_try_init(|| File::open("/dev/urandom"))
        .and_then(|mut dev| dev.read_exact(bytes))
