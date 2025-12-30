// Generated macro for byteset_create (function)
macro_rules! Depcrate_findbyteset_create {
() => {
// Module: crate::find
// Provides: {"byteset_create"}
// Dependencies: {}
fn byteset_create (chars : & [char]) -> u64 { chars . iter () . fold (0 , | a , & ch | (1 << (ch as u8 & 0x3f)) | a) }
};
}
