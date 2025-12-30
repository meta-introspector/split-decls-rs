// Generated macro for IoRead (struct)
macro_rules! Depcrate_readIoRead {
() => {
// Module: crate::read
// Provides: {"IoRead"}
// Dependencies: {}
# [doc = " JSON input source that reads from a std::io input stream."] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub struct IoRead < R > where R : io :: Read , { iter : LineColIterator < io :: Bytes < R > > , # [doc = " Temporary storage of peeked byte."] ch : Option < u8 > , # [cfg (feature = "raw_value")] raw_buffer : Option < Vec < u8 > > , }
};
}
