macro_rules! deps {
    () => {
        Section!();
        U64!();
        U32!();
    };
}

macro_rules! SectionHeader64 {
    () => {
        deps!();
        # [doc = " Section header."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SectionHeader64 { # [doc = " Section name."] pub s_name : [u8 ; 8] , # [doc = " Physical address."] pub s_paddr : U64 < BE > , # [doc = " Virtual address (same as physical address)."] pub s_vaddr : U64 < BE > , # [doc = " Section size."] pub s_size : U64 < BE > , # [doc = " Offset in file to raw data for section."] pub s_scnptr : U64 < BE > , # [doc = " Offset in file to relocation entries for section."] pub s_relptr : U64 < BE > , # [doc = " Offset in file to line number entries for section."] pub s_lnnoptr : U64 < BE > , # [doc = " Number of relocation entries."] pub s_nreloc : U32 < BE > , # [doc = " Number of line number entries."] pub s_nlnno : U32 < BE > , # [doc = " Flags to define the section type."] pub s_flags : U32 < BE > , # [doc = " Reserved."] pub s_reserve : U32 < BE > , }
    };
}

SectionHeader64!();