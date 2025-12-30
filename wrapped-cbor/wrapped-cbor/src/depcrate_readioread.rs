// Generated macro for IoRead (struct)
macro_rules! Depcrate_readIoRead {
() => {
// Module: crate::read
// Provides: {"IoRead"}
// Dependencies: {}
# [doc = " CBOR input source that reads from a std::io input stream."] # [cfg (feature = "std")] # [derive (Debug)] pub struct IoRead < R > where R : io :: Read , { reader : OffsetReader < R > , scratch : Vec < u8 > , ch : Option < u8 > , }
};
}
