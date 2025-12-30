// Generated macro for DataInCodeEntry (struct)
macro_rules! Depcrate_machoDataInCodeEntry {
() => {
// Module: crate::macho
// Provides: {"DataInCodeEntry"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DataInCodeEntry < E : Endian > { # [doc = " from mach_header to start of data range"] pub offset : U32 < E > , # [doc = " number of bytes in data range"] pub length : U16 < E > , # [doc = " a DICE_KIND_* value"] pub kind : U16 < E > , }
};
}
