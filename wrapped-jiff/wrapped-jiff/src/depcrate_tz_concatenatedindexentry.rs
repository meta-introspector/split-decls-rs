// Generated macro for IndexEntry (struct)
macro_rules! Depcrate_tz_concatenatedIndexEntry {
() => {
// Module: crate::tz::concatenated
// Provides: {"IndexEntry"}
// Dependencies: {}
# [doc = " A view into a single index entry in the index block of concatenated TZif"] # [doc = " data."] # [doc = ""] # [doc = " If we had safe transmute, it would be much nicer to define this as"] # [doc = ""] # [doc = " ```text"] # [doc = " #[derive(Clone, Copy)]"] # [doc = " #[repr(transparent, align(1))]"] # [doc = " struct IndexEntry {"] # [doc = "     name: [u8; 40],"] # [doc = "     start: u32,"] # [doc = "     len: u32,"] # [doc = "     _raw_utc_offset: u32, // we don't use this here"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " And probably implement a trait asserting that this is plain old data (or"] # [doc = " derive it safely). And then we could cast `&[u8]` to `&[IndexEntry]`"] # [doc = " safely and access the individual fields as is. We could do this today,"] # [doc = " but not in safe code. And since this isn't performance critical, it's just"] # [doc = " not worth flagging this code as potentially containing undefined behavior."] # [derive (Clone , Copy)] struct IndexEntry < 'a > (& 'a [u8]) ;
};
}
