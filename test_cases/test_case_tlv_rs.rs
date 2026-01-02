// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_thread_pool/src/tlv.rs
// Error: expected square brackets
// Problematic line: line 9


thread_local!(pub static TLV: Cell<*const ()> = const { Cell::new(ptr::null()) });

#[derive(Copy, Clone)]
pub(crate) struct Tlv(pub(crate) *const ());

impl Tlv {
