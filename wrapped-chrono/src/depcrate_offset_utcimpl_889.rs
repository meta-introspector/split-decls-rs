// Generated macro for impl_889 (impl)
macro_rules! Depcrate_offset_utcimpl_889 {
() => {
// Module: crate::offset::utc
// Provides: {"impl_889"}
// Dependencies: {}
impl TimeZone for Utc { type Offset = Utc ; fn from_offset (_state : & Utc) -> Utc { Utc } fn offset_from_local_date (& self , _local : & NaiveDate) -> MappedLocalTime < Utc > { MappedLocalTime :: Single (Utc) } fn offset_from_local_datetime (& self , _local : & NaiveDateTime) -> MappedLocalTime < Utc > { MappedLocalTime :: Single (Utc) } fn offset_from_utc_date (& self , _utc : & NaiveDate) -> Utc { Utc } fn offset_from_utc_datetime (& self , _utc : & NaiveDateTime) -> Utc { Utc } }
};
}
