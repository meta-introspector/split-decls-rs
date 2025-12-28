macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! SymtabCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SymtabCommand < E : Endian > { # [doc = " LC_SYMTAB"] pub cmd : U32 < E > , # [doc = " sizeof(struct SymtabCommand)"] pub cmdsize : U32 < E > , # [doc = " symbol table offset"] pub symoff : U32 < E > , # [doc = " number of symbol table entries"] pub nsyms : U32 < E > , # [doc = " string table offset"] pub stroff : U32 < E > , # [doc = " string table size in bytes"] pub strsize : U32 < E > , }
    };
}

SymtabCommand!();