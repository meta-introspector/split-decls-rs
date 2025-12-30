// Generated macro for impl_1374 (impl)
macro_rules! Depcrate_serde_utilsimpl_1374 {
() => {
// Module: crate::serde_utils
// Provides: {"impl_1374"}
// Dependencies: {}
impl < 'de , F , T , R > MappedSequenceVisitor < T , R , F > where T : Deserialize < 'de > , F : Fn (T) -> Result < R , & 'static str > , { pub fn new (f : F) -> Self { MappedSequenceVisitor { f , marker : PhantomData , } } }
};
}
