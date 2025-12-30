// Generated macro for impl_19 (impl)
macro_rules! Depcrate_forksafeimpl_19 {
() => {
// Module: crate::forksafe
// Provides: {"impl_19"}
// Dependencies: {}
impl ForksafeTempfile { pub fn new (tempfile : NamedTempFile , cleanup : AutoRemove , mode : handle :: Mode) -> Self { use handle :: Mode :: * ; ForksafeTempfile { inner : match mode { Closed => TempfileOrTemppath :: Temppath (tempfile . into_temp_path ()) , Writable => TempfileOrTemppath :: Tempfile (tempfile) , } , cleanup , owning_process_id : std :: process :: id () , } } }
};
}
