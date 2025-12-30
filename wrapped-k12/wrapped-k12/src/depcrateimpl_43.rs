// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl XofReader for KangarooTwelveReader { # [inline] fn read (& mut self , buffer : & mut [u8]) { let Self { core , buffer : buf } = self ; buf . read (buffer , | block | * block = core . read_block ()) ; } }
};
}
