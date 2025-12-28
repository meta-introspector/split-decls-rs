macro_rules! deps {
    () => {
        ParserRange!();
        NodeRange!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl NodeRange { pub fn new (ParserRange (parser_range) : ParserRange , start_pos : u32) -> NodeRange { assert ! (! parser_range . is_empty ()) ; assert ! (parser_range . start >= start_pos) ; NodeRange ((parser_range . start - start_pos) .. (parser_range . end - start_pos)) } }
    };
}

impl_433!();