// Generated macro for impl_3193 (impl)
macro_rules! Depcrate_pg_connection_copyimpl_3193 {
() => {
// Module: crate::pg::connection::copy
// Provides: {"impl_3193"}
// Dependencies: {}
impl < 'conn > CopyToBuffer < 'conn > { pub (super) fn new (conn : & 'conn mut RawConnection , result : PgResult) -> Self { Self { conn , ptr : std :: ptr :: null_mut () , offset : 0 , len : 0 , result , } } # [allow (unsafe_code)] pub (crate) fn data_slice (& self) -> & [u8] { if ! self . ptr . is_null () && self . offset < self . len { let slice = unsafe { std :: slice :: from_raw_parts (self . ptr as * const u8 , self . len - 1) } ; & slice [self . offset ..] } else { & [] } } pub (crate) fn get_result (& self) -> & PgResult { & self . result } }
};
}
