macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! DysymtabCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DysymtabCommand < E : Endian > { # [doc = " LC_DYSYMTAB"] pub cmd : U32 < E > , # [doc = " sizeof(struct DysymtabCommand)"] pub cmdsize : U32 < E > , # [doc = " index to local symbols"] pub ilocalsym : U32 < E > , # [doc = " number of local symbols"] pub nlocalsym : U32 < E > , # [doc = " index to externally defined symbols"] pub iextdefsym : U32 < E > , # [doc = " number of externally defined symbols"] pub nextdefsym : U32 < E > , # [doc = " index to undefined symbols"] pub iundefsym : U32 < E > , # [doc = " number of undefined symbols"] pub nundefsym : U32 < E > , # [doc = " file offset to table of contents"] pub tocoff : U32 < E > , # [doc = " number of entries in table of contents"] pub ntoc : U32 < E > , # [doc = " file offset to module table"] pub modtaboff : U32 < E > , # [doc = " number of module table entries"] pub nmodtab : U32 < E > , # [doc = " offset to referenced symbol table"] pub extrefsymoff : U32 < E > , # [doc = " number of referenced symbol table entries"] pub nextrefsyms : U32 < E > , # [doc = " file offset to the indirect symbol table"] pub indirectsymoff : U32 < E > , # [doc = " number of indirect symbol table entries"] pub nindirectsyms : U32 < E > , # [doc = " offset to external relocation entries"] pub extreloff : U32 < E > , # [doc = " number of external relocation entries"] pub nextrel : U32 < E > , # [doc = " offset to local relocation entries"] pub locreloff : U32 < E > , # [doc = " number of local relocation entries"] pub nlocrel : U32 < E > , }
    };
}

DysymtabCommand!();