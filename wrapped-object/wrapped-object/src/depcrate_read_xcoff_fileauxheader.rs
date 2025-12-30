// Generated macro for AuxHeader (trait)
macro_rules! Depcrate_read_xcoff_fileAuxHeader {
() => {
// Module: crate::read::xcoff::file
// Provides: {"AuxHeader"}
// Dependencies: {}
# [doc = " A trait for generic access to [`xcoff::AuxHeader32`] and [`xcoff::AuxHeader64`]."] # [allow (missing_docs)] pub trait AuxHeader : Debug + Pod { type Word : Into < u64 > ; fn o_mflag (& self) -> u16 ; fn o_vstamp (& self) -> u16 ; fn o_tsize (& self) -> Self :: Word ; fn o_dsize (& self) -> Self :: Word ; fn o_bsize (& self) -> Self :: Word ; fn o_entry (& self) -> Self :: Word ; fn o_text_start (& self) -> Self :: Word ; fn o_data_start (& self) -> Self :: Word ; fn o_toc (& self) -> Self :: Word ; fn o_snentry (& self) -> u16 ; fn o_sntext (& self) -> u16 ; fn o_sndata (& self) -> u16 ; fn o_sntoc (& self) -> u16 ; fn o_snloader (& self) -> u16 ; fn o_snbss (& self) -> u16 ; fn o_algntext (& self) -> u16 ; fn o_algndata (& self) -> u16 ; fn o_modtype (& self) -> u16 ; fn o_cpuflag (& self) -> u8 ; fn o_cputype (& self) -> u8 ; fn o_maxstack (& self) -> Self :: Word ; fn o_maxdata (& self) -> Self :: Word ; fn o_debugger (& self) -> u32 ; fn o_textpsize (& self) -> u8 ; fn o_datapsize (& self) -> u8 ; fn o_stackpsize (& self) -> u8 ; fn o_flags (& self) -> u8 ; fn o_sntdata (& self) -> u16 ; fn o_sntbss (& self) -> u16 ; fn o_x64flags (& self) -> Option < u16 > ; }
};
}
