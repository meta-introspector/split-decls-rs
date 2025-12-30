// Generated macro for IoReader (struct)
macro_rules! Depcrate_deIoReader {
() => {
// Module: crate::de
// Provides: {"IoReader"}
// Dependencies: {}
# [doc = " XML input source that reads from a std::io input stream."] # [doc = ""] # [doc = " You cannot create it, it is created automatically when you call"] # [doc = " [`Deserializer::from_reader`]"] pub struct IoReader < R : BufRead > { reader : NsReader < R > , buf : Vec < u8 > , }
};
}
