macro_rules! deps {
    () => {
        HexDisplay!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl HexDisplay for str { # [allow (unused_variables)] fn to_hex (& self , chunk_size : usize) -> String { self . to_hex_from (chunk_size , 0) } # [allow (unused_variables)] fn to_hex_from (& self , chunk_size : usize , from : usize) -> String { self . as_bytes () . to_hex_from (chunk_size , from) } }
    };
}

impl_371!();