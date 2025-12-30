// Generated macro for impl_1147 (impl)
macro_rules! Depcrate_fs_memimpl_1147 {
() => {
// Module: crate::fs::mem
// Provides: {"impl_1147"}
// Dependencies: {}
impl RamFile { pub fn new (mode : AccessPermission) -> Self { let microseconds = arch :: kernel :: systemtime :: now_micros () ; let t = timespec :: from_usec (microseconds as i64) ; let attr = FileAttr { st_mode : mode | AccessPermission :: S_IFREG , st_atim : t , st_mtim : t , st_ctim : t , .. Default :: default () } ; Self { data : Arc :: new (RwLock :: new (RamFileInner :: new (attr))) , } } }
};
}
