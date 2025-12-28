macro_rules! Address {
    () => {
        # [doc = " An address."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum Address { # [doc = " A fixed address that does not require relocation."] Constant (u64) , # [doc = " An address that is relative to a symbol which may be relocated."] Symbol { # [doc = " The symbol that the address is relative to."] # [doc = ""] # [doc = " The meaning of this value is decided by the writer, but"] # [doc = " will typically be an index into a symbol table."] symbol : usize , # [doc = " The offset of the address relative to the symbol."] # [doc = ""] # [doc = " This will typically be used as the addend in a relocation."] addend : i64 , } , }
    };
}

Address!();