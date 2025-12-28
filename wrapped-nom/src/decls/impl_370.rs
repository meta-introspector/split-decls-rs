macro_rules! deps {
    () => {
        HexDisplay!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl HexDisplay for [u8] { # [allow (unused_variables)] fn to_hex (& self , chunk_size : usize) -> String { self . to_hex_from (chunk_size , 0) } # [allow (unused_variables)] fn to_hex_from (& self , chunk_size : usize , from : usize) -> String { let mut v = Vec :: with_capacity (self . len () * 3) ; let mut i = from ; for chunk in self . chunks (chunk_size) { let s = format ! ("{:08x}" , i) ; for & ch in s . as_bytes () . iter () { v . push (ch) ; } v . push (b'\t') ; i += chunk_size ; for & byte in chunk { v . push (CHARS [(byte >> 4) as usize]) ; v . push (CHARS [(byte & 0xf) as usize]) ; v . push (b' ') ; } if chunk_size > chunk . len () { for j in 0 .. (chunk_size - chunk . len ()) { v . push (b' ') ; v . push (b' ') ; v . push (b' ') ; } } v . push (b'\t') ; for & byte in chunk { if matches ! (byte , 32 ..= 126 | 128 ..= 255) { v . push (byte) ; } else { v . push (b'.') ; } } v . push (b'\n') ; } String :: from_utf8_lossy (& v [..]) . into_owned () } }
    };
}

impl_370!()