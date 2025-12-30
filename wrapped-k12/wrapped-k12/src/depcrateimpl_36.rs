// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl Update for KangarooTwelve < '_ > { fn update (& mut self , data : & [u8]) { let Self { core , buffer } = self ; buffer . digest_blocks (data , | blocks | core . update_blocks (blocks)) ; } }
};
}
