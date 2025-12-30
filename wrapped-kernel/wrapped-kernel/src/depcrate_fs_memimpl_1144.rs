// Generated macro for impl_1144 (impl)
macro_rules! Depcrate_fs_memimpl_1144 {
() => {
// Module: crate::fs::mem
// Provides: {"impl_1144"}
// Dependencies: {}
impl RomFile { pub fn new (data : & 'static [u8] , mode : AccessPermission) -> Self { let microseconds = arch :: kernel :: systemtime :: now_micros () ; let t = timespec :: from_usec (microseconds as i64) ; let attr = FileAttr { st_size : data . len () . try_into () . unwrap () , st_mode : mode | AccessPermission :: S_IFREG , st_atim : t , st_mtim : t , st_ctim : t , .. Default :: default () } ; Self { data : Arc :: new (RwLock :: new (RomFileInner :: new (data , attr))) , } } }
};
}
