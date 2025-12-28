macro_rules! deps {
    () => {
        MachO!();
        Section!();
    };
}

macro_rules! SectionFlags {
    () => {
        deps!();
        # [doc = " Section flags that are specific to each file format."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum SectionFlags { # [doc = " No section flags."] None , # [doc = " ELF section flags."] Elf { # [doc = " `sh_flags` field in the section header."] sh_flags : u64 , } , # [doc = " Mach-O section flags."] MachO { # [doc = " `flags` field in the section header."] flags : u32 , } , # [doc = " COFF section flags."] Coff { # [doc = " `Characteristics` field in the section header."] characteristics : u32 , } , # [doc = " XCOFF section flags."] Xcoff { # [doc = " `s_flags` field in the section header."] s_flags : u32 , } , }
    };
}

SectionFlags!()