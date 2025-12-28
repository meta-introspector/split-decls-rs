macro_rules! deps {
    () => {
        Address!();
        CallFrameInstruction!();
        Encoding!();
        Register!();
    };
}

macro_rules! CommonInformationEntry {
    () => {
        deps!();
        # [doc = " A common information entry. This contains information that is shared between FDEs."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct CommonInformationEntry { encoding : Encoding , # [doc = " A constant that is factored out of code offsets."] # [doc = ""] # [doc = " This should be set to the minimum instruction length."] # [doc = " Writing a code offset that is not a multiple of this factor will generate an error."] code_alignment_factor : u8 , # [doc = " A constant that is factored out of data offsets."] # [doc = ""] # [doc = " This should be set to the minimum data alignment for the frame."] # [doc = " Writing a data offset that is not a multiple of this factor will generate an error."] data_alignment_factor : i8 , # [doc = " The return address register. This might not correspond to an actual machine register."] return_address_register : Register , # [doc = " The address of the personality function and its encoding."] pub personality : Option < (constants :: DwEhPe , Address) > , # [doc = " The encoding to use for the LSDA address in FDEs."] # [doc = ""] # [doc = " If set then all FDEs which use this CIE must have a LSDA address."] pub lsda_encoding : Option < constants :: DwEhPe > , # [doc = " The encoding to use for addresses in FDEs."] pub fde_address_encoding : constants :: DwEhPe , # [doc = " True for signal trampolines."] pub signal_trampoline : bool , # [doc = " The initial instructions upon entry to this function."] instructions : Vec < CallFrameInstruction > , }
    };
}

CommonInformationEntry!();