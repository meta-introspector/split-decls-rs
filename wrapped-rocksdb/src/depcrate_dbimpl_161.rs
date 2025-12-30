// Generated macro for impl_161 (impl)
macro_rules! Depcrate_dbimpl_161 {
() => {
// Module: crate::db
// Provides: {"impl_161"}
// Dependencies: {}
impl < T : ThreadMode , I : DBInner > fmt :: Debug for DBCommon < T , I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "RocksDB {{ path: {:?} }}" , self . path ()) } }
};
}
