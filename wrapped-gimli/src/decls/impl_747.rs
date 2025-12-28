macro_rules! deps {
    () => {
        Encoding!();
        LineRow!();
        LineEncoding!();
    };
}

macro_rules! impl_747 {
    () => {
        deps!();
        impl LineRow { # [doc = " Return the initial state as specified in the DWARF standard."] fn initial_state (encoding : Encoding , line_encoding : LineEncoding) -> Self { LineRow { address_offset : 0 , op_index : 0 , file : FileId :: initial_state (encoding . version) , line : 1 , column : 0 , discriminator : 0 , is_statement : line_encoding . default_is_stmt , basic_block : false , prologue_end : false , epilogue_begin : false , isa : 0 , } } }
    };
}

impl_747!()