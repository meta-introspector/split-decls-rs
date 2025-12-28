macro_rules! deps {
    () => {
        MangledName!();
        BorrowedSymbol!();
        OwnedSymbol!();
        SubstitutionTable!();
    };
}

macro_rules! Symbol {
    () => {
        deps!();
        # [doc = " A mangled symbol that has been parsed into an AST."] # [doc = ""] # [doc = " This is generic over some storage type `T` which can be either owned or"] # [doc = " borrowed. See the `OwnedSymbol` and `BorrowedSymbol` type aliases."] # [derive (Clone , Debug , PartialEq)] pub struct Symbol < T > { raw : T , substitutions : subs :: SubstitutionTable , parsed : ast :: MangledName , }
    };
}

Symbol!()