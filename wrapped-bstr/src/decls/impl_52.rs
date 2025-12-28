macro_rules! deps {
    () => {
        UnescapeBytes!();
        UnescapeState!();
        Bytes!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I : Iterator < Item = char > > Iterator for UnescapeBytes < I > { type Item = u8 ; fn next (& mut self) -> Option < u8 > { use self :: UnescapeState :: * ; loop { match self . state { Start => { let ch = self . it . next () ? ; match ch { '\\' => { self . state = Escape ; } ch => { self . state = UnescapeState :: bytes (& [] , ch) ; } } } Bytes { buf , mut cur , len } => { let byte = buf [cur] ; cur += 1 ; if cur >= len { self . state = Start ; } else { self . state = Bytes { buf , cur , len } ; } return Some (byte) ; } Escape => { let ch = match self . it . next () { Some (ch) => ch , None => { self . state = Start ; return Some (b'\\') ; } } ; match ch { '0' => { self . state = Start ; return Some (b'\x00') ; } '\\' => { self . state = Start ; return Some (b'\\') ; } 'r' => { self . state = Start ; return Some (b'\r') ; } 'n' => { self . state = Start ; return Some (b'\n') ; } 't' => { self . state = Start ; return Some (b'\t') ; } 'x' => { self . state = HexFirst ; } ch => { self . state = UnescapeState :: bytes (& [b'\\'] , ch) ; } } } HexFirst => { let ch = match self . it . next () { Some (ch) => ch , None => { self . state = UnescapeState :: bytes_raw (& [b'x']) ; return Some (b'\\') ; } } ; match ch { '0' ..= '9' | 'A' ..= 'F' | 'a' ..= 'f' => { self . state = HexSecond (ch) ; } ch => { self . state = UnescapeState :: bytes (& [b'x'] , ch) ; return Some (b'\\') ; } } } HexSecond (first) => { let second = match self . it . next () { Some (ch) => ch , None => { self . state = UnescapeState :: bytes (& [b'x'] , first) ; return Some (b'\\') ; } } ; match second { '0' ..= '9' | 'A' ..= 'F' | 'a' ..= 'f' => { self . state = Start ; let hinybble = char_to_hexdigit (first) ; let lonybble = char_to_hexdigit (second) ; let byte = hinybble << 4 | lonybble ; return Some (byte) ; } ch => { self . state = UnescapeState :: bytes2 (& [b'x'] , first , ch) ; return Some (b'\\') ; } } } } } } }
    };
}

impl_52!();