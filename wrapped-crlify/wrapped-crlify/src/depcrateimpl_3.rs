// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl < W : Write > BufWriterWithLineEndingFix < W > { pub fn new (inner : W) -> Self { Self { inner : BufWriter :: with_capacity (4096 , inner) , # [cfg (windows)] last_written : None , } } }
};
}
