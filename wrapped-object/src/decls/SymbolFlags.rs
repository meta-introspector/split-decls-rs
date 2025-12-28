macro_rules! deps {
    () => {
        MachO!();
        CoffSection!();
        Symbol!();
        Section!();
    };
}

macro_rules! SymbolFlags {
    () => {
        deps!();
        # [doc = " Symbol flags that are specific to each file format."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum SymbolFlags < Section , Symbol > { # [doc = " No symbol flags."] None , # [doc = " ELF symbol flags."] Elf { # [doc = " `st_info` field in the ELF symbol."] st_info : u8 , # [doc = " `st_other` field in the ELF symbol."] st_other : u8 , } , # [doc = " Mach-O symbol flags."] MachO { # [doc = " `n_desc` field in the Mach-O symbol."] n_desc : u16 , } , # [doc = " COFF flags for a section symbol."] CoffSection { # [doc = " `Selection` field in the auxiliary symbol for the section."] selection : u8 , # [doc = " `Number` field in the auxiliary symbol for the section."] associative_section : Option < Section > , } , # [doc = " XCOFF symbol flags."] Xcoff { # [doc = " `n_sclass` field in the XCOFF symbol."] n_sclass : u8 , # [doc = " `x_smtyp` field in the CSECT auxiliary symbol."] # [doc = ""] # [doc = " Only valid if `n_sclass` is `C_EXT`, `C_WEAKEXT`, or `C_HIDEXT`."] x_smtyp : u8 , # [doc = " `x_smclas` field in the CSECT auxiliary symbol."] # [doc = ""] # [doc = " Only valid if `n_sclass` is `C_EXT`, `C_WEAKEXT`, or `C_HIDEXT`."] x_smclas : u8 , # [doc = " The containing csect for the symbol."] # [doc = ""] # [doc = " Only valid if `x_smtyp` is `XTY_LD`."] containing_csect : Option < Symbol > , } , }
    };
}

SymbolFlags!()