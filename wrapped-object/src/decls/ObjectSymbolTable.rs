macro_rules! deps {
    () => {
        Item!();
        Symbol!();
        Object!();
        SymbolIterator!();
        SymbolIndex!();
        Result!();
        ObjectSymbol!();
    };
}

macro_rules! ObjectSymbolTable {
    () => {
        deps!();
        # [doc = " A symbol table in an [`Object`]."] # [doc = ""] # [doc = " This trait is part of the unified read API."] pub trait ObjectSymbolTable < 'data > : read :: private :: Sealed { # [doc = " A symbol table entry."] type Symbol : ObjectSymbol < 'data > ; # [doc = " An iterator for the symbols in a symbol table."] type SymbolIterator : Iterator < Item = Self :: Symbol > ; # [doc = " Get an iterator for the symbols in the table."] # [doc = ""] # [doc = " This may skip over symbols that are malformed or unsupported."] fn symbols (& self) -> Self :: SymbolIterator ; # [doc = " Get the symbol at the given index."] # [doc = ""] # [doc = " The meaning of the index depends on the object file."] # [doc = ""] # [doc = " Returns an error if the index is invalid."] fn symbol_by_index (& self , index : SymbolIndex) -> Result < Self :: Symbol > ; }
    };
}

ObjectSymbolTable!();