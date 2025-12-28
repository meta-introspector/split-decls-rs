macro_rules! deps {
    () => {
        Pointer!();
    };
}

macro_rules! macro_93 {
    () => {
        deps!();
        dw ! (# [doc = " Pointer encoding used by `.eh_frame`."] # [doc = ""] # [doc = " The four lower bits describe the"] # [doc = " format of the pointer, the upper four bits describe how the encoding should"] # [doc = " be applied."] # [doc = ""] # [doc = " Defined in `<https://refspecs.linuxfoundation.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html>`"] DwEhPe (u8) { DW_EH_PE_uleb128 = 0x1 , DW_EH_PE_udata2 = 0x2 , DW_EH_PE_udata4 = 0x3 , DW_EH_PE_udata8 = 0x4 , DW_EH_PE_sleb128 = 0x9 , DW_EH_PE_sdata2 = 0x0a , DW_EH_PE_sdata4 = 0x0b , DW_EH_PE_sdata8 = 0x0c , DW_EH_PE_pcrel = 0x10 , DW_EH_PE_textrel = 0x20 , DW_EH_PE_datarel = 0x30 , DW_EH_PE_funcrel = 0x40 , DW_EH_PE_aligned = 0x50 , DW_EH_PE_indirect = 0x80 , DW_EH_PE_absptr = 0x0 , DW_EH_PE_omit = 0xff , }) ;
    };
}

macro_93!()