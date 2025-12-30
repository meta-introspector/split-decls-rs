// Generated macro for FixedLength (struct)
macro_rules! Depcrate_ivFixedLength {
() => {
// Module: crate::iv
// Provides: {"FixedLength"}
// Dependencies: {}
# [doc = " An initialization vector that must be unique for the lifetime of the associated key"] # [doc = " it is used with."] pub struct FixedLength < const L : usize > ([u8 ; L]) ;
};
}
