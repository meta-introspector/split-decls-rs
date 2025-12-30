// Generated macro for as_bytes (function)
macro_rules! Depcrateas_bytes {
() => {
// Module: crate
// Provides: {"as_bytes"}
// Dependencies: {}
fn as_bytes (value : & HSTRING) -> & [u8] { unsafe { core :: slice :: from_raw_parts (value . as_ptr () as * const _ , (value . len () + 1) * 2) } }
};
}
