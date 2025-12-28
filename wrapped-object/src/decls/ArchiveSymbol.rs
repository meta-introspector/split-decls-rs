macro_rules! deps {
    () => {
        ArchiveOffset!();
    };
}

macro_rules! ArchiveSymbol {
    () => {
        deps!();
        # [doc = " A symbol in the archive symbol table."] # [doc = ""] # [doc = " This is used to find the member containing the symbol."] # [derive (Debug , Clone , Copy)] pub struct ArchiveSymbol < 'data > { name : & 'data [u8] , offset : ArchiveOffset , }
    };
}

ArchiveSymbol!()