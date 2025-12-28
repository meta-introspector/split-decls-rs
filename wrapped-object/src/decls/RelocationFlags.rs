macro_rules! deps {
    () => {
        RelocationKind!();
        RelocationEncoding!();
        Relocation!();
        MachO!();
    };
}

macro_rules! RelocationFlags {
    () => {
        deps!();
        # [doc = " Relocation fields that are specific to each file format and architecture."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum RelocationFlags { # [doc = " Format independent representation."] Generic { # [doc = " The operation used to calculate the result of the relocation."] kind : RelocationKind , # [doc = " Information about how the result of the relocation operation is encoded in the place."] encoding : RelocationEncoding , # [doc = " The size in bits of the place of relocation."] size : u8 , } , # [doc = " ELF relocation fields."] Elf { # [doc = " `r_type` field in the ELF relocation."] r_type : u32 , } , # [doc = " Mach-O relocation fields."] MachO { # [doc = " `r_type` field in the Mach-O relocation."] r_type : u8 , # [doc = " `r_pcrel` field in the Mach-O relocation."] r_pcrel : bool , # [doc = " `r_length` field in the Mach-O relocation."] r_length : u8 , } , # [doc = " COFF relocation fields."] Coff { # [doc = " `typ` field in the COFF relocation."] typ : u16 , } , # [doc = " XCOFF relocation fields."] Xcoff { # [doc = " `r_rtype` field in the XCOFF relocation."] r_rtype : u8 , # [doc = " `r_rsize` field in the XCOFF relocation."] r_rsize : u8 , } , }
    };
}

RelocationFlags!()