macro_rules! deps {
    () => {
        U16!();
        U32!();
        Section!();
    };
}

macro_rules! SectionHeader32 {
    () => {
        deps!();
        # [doc = " Section header."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SectionHeader32 { # [doc = " Section name."] pub s_name : [u8 ; 8] , # [doc = " Physical address."] pub s_paddr : U32 < BE > , # [doc = " Virtual address (same as physical address)."] pub s_vaddr : U32 < BE > , # [doc = " Section size."] pub s_size : U32 < BE > , # [doc = " Offset in file to raw data for section."] pub s_scnptr : U32 < BE > , # [doc = " Offset in file to relocation entries for section."] pub s_relptr : U32 < BE > , # [doc = " Offset in file to line number entries for section."] pub s_lnnoptr : U32 < BE > , # [doc = " Number of relocation entries."] pub s_nreloc : U16 < BE > , # [doc = " Number of line number entries."] pub s_nlnno : U16 < BE > , # [doc = " Flags to define the section type."] pub s_flags : U32 < BE > , }
    };
}

SectionHeader32!()