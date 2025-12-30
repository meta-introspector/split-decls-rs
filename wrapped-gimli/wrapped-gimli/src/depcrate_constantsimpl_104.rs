// Generated macro for impl_104 (impl)
macro_rules! Depcrate_constantsimpl_104 {
() => {
// Module: crate::constants
// Provides: {"impl_104"}
// Dependencies: {}
impl DwEhPe { # [doc = " Get the pointer encoding's format."] # [inline] pub fn format (self) -> DwEhPe { DwEhPe (self . 0 & DW_EH_PE_FORMAT_MASK) } # [doc = " Get the pointer encoding's application."] # [inline] pub fn application (self) -> DwEhPe { DwEhPe (self . 0 & DW_EH_PE_APPLICATION_MASK) } # [doc = " Is this encoding the absent pointer encoding?"] # [inline] pub fn is_absent (self) -> bool { self == DW_EH_PE_omit } # [doc = " Is this coding indirect? If so, its encoded value is the address of the"] # [doc = " real pointer result, not the pointer result itself."] # [inline] pub fn is_indirect (self) -> bool { self . 0 & DW_EH_PE_indirect . 0 != 0 } # [doc = " Is this a known, valid pointer encoding?"] pub fn is_valid_encoding (self) -> bool { if self . is_absent () { return true ; } match self . format () { DW_EH_PE_absptr | DW_EH_PE_uleb128 | DW_EH_PE_udata2 | DW_EH_PE_udata4 | DW_EH_PE_udata8 | DW_EH_PE_sleb128 | DW_EH_PE_sdata2 | DW_EH_PE_sdata4 | DW_EH_PE_sdata8 => { } _ => return false , } match self . application () { DW_EH_PE_absptr | DW_EH_PE_pcrel | DW_EH_PE_textrel | DW_EH_PE_datarel | DW_EH_PE_funcrel | DW_EH_PE_aligned => { } _ => return false , } true } }
};
}
