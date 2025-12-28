macro_rules! deps {
    () => {
        Section!();
        U16!();
        Version!();
        U32!();
    };
}

macro_rules! AuxHeader32 {
    () => {
        deps!();
        # [doc = " The auxiliary header immediately following file header. If the value of the"] # [doc = " f_opthdr field in the file header is 0, the auxiliary header does not exist."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct AuxHeader32 { # [doc = " Flags."] pub o_mflag : U16 < BE > , # [doc = " Version."] pub o_vstamp : U16 < BE > , # [doc = " Text size in bytes."] pub o_tsize : U32 < BE > , # [doc = " Initialized data size in bytes."] pub o_dsize : U32 < BE > , # [doc = " Uninitialized data size in bytes."] pub o_bsize : U32 < BE > , # [doc = " Entry point descriptor (virtual address)."] pub o_entry : U32 < BE > , # [doc = " Base address of text (virtual address)."] pub o_text_start : U32 < BE > , # [doc = " Base address of data (virtual address)."] pub o_data_start : U32 < BE > , # [doc = " Address of TOC anchor."] pub o_toc : U32 < BE > , # [doc = " Section number for entry point."] pub o_snentry : U16 < BE > , # [doc = " Section number for .text."] pub o_sntext : U16 < BE > , # [doc = " Section number for .data."] pub o_sndata : U16 < BE > , # [doc = " Section number for TOC."] pub o_sntoc : U16 < BE > , # [doc = " Section number for loader data."] pub o_snloader : U16 < BE > , # [doc = " Section number for .bss."] pub o_snbss : U16 < BE > , # [doc = " Maximum alignment for .text."] pub o_algntext : U16 < BE > , # [doc = " Maximum alignment for .data."] pub o_algndata : U16 < BE > , # [doc = " Module type field."] pub o_modtype : U16 < BE > , # [doc = " Bit flags - cpu types of objects."] pub o_cpuflag : u8 , # [doc = " Reserved for CPU type."] pub o_cputype : u8 , # [doc = " Maximum stack size allowed (bytes)."] pub o_maxstack : U32 < BE > , # [doc = " Maximum data size allowed (bytes)."] pub o_maxdata : U32 < BE > , # [doc = " Reserved for debuggers."] pub o_debugger : U32 < BE > , # [doc = " Requested text page size."] pub o_textpsize : u8 , # [doc = " Requested data page size."] pub o_datapsize : u8 , # [doc = " Requested stack page size."] pub o_stackpsize : u8 , # [doc = " Flags and thread-local storage alignment."] pub o_flags : u8 , # [doc = " Section number for .tdata."] pub o_sntdata : U16 < BE > , # [doc = " Section number for .tbss."] pub o_sntbss : U16 < BE > , }
    };
}

AuxHeader32!();