// Generated macro for GzHeader (struct)
macro_rules! Depcrate_gzGzHeader {
() => {
// Module: crate::gz
// Provides: {"GzHeader"}
// Dependencies: {}
# [doc = " A structure representing the header of a gzip stream."] # [doc = ""] # [doc = " The header can contain metadata about the file that was compressed, if"] # [doc = " present."] # [derive (PartialEq , Clone , Debug , Default)] pub struct GzHeader { extra : Option < Vec < u8 > > , filename : Option < Vec < u8 > > , comment : Option < Vec < u8 > > , operating_system : u8 , mtime : u32 , }
};
}
