// Generated macro for StringTable (struct)
macro_rules! Depcrate_read_utilStringTable {
() => {
// Module: crate::read::util
// Provides: {"StringTable"}
// Dependencies: {}
# [doc = " A table of zero-terminated strings."] # [doc = ""] # [doc = " This is used by most file formats for strings such as section names and symbol names."] # [derive (Debug , Clone , Copy)] pub struct StringTable < 'data , R = & 'data [u8] > where R : ReadRef < 'data > , { data : Option < R > , start : u64 , end : u64 , marker : PhantomData < & 'data () > , }
};
}
