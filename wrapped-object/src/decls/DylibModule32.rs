macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! DylibModule32 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DylibModule32 < E : Endian > { # [doc = " the module name (index into string table)"] pub module_name : U32 < E > , # [doc = " index into externally defined symbols"] pub iextdefsym : U32 < E > , # [doc = " number of externally defined symbols"] pub nextdefsym : U32 < E > , # [doc = " index into reference symbol table"] pub irefsym : U32 < E > , # [doc = " number of reference symbol table entries"] pub nrefsym : U32 < E > , # [doc = " index into symbols for local symbols"] pub ilocalsym : U32 < E > , # [doc = " number of local symbols"] pub nlocalsym : U32 < E > , # [doc = " index into external relocation entries"] pub iextrel : U32 < E > , # [doc = " number of external relocation entries"] pub nextrel : U32 < E > , # [doc = " low 16 bits are the index into the init section, high 16 bits are the index into the term section"] pub iinit_iterm : U32 < E > , # [doc = " low 16 bits are the number of init section entries, high 16 bits are the number of term section entries"] pub ninit_nterm : U32 < E > , # [doc = " for this module address of the start of the (__OBJC,__module_info) section"] pub objc_module_info_addr : U32 < E > , # [doc = " for this module size of the (__OBJC,__module_info) section"] pub objc_module_info_size : U32 < E > , }
    };
}

DylibModule32!()