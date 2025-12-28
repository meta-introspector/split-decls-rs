macro_rules! deps {
    () => {
        U32Bytes!();
        U16Bytes!();
        Symbol!();
    };
}

macro_rules! ImageLinenumber {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageLinenumber { # [doc = " Symbol table index of function name if Linenumber is 0."] # [doc = " Otherwise virtual address of line number."] pub symbol_table_index_or_virtual_address : U32Bytes < LE > , # [doc = " Line number."] pub linenumber : U16Bytes < LE > , }
    };
}

ImageLinenumber!()