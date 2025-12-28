macro_rules! deps {
    () => {
        EscapeState!();
        EscapeBytes!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a > Iterator for EscapeBytes < 'a > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { use self :: EscapeState :: * ; match self . state { Start => { let byte = match crate :: decode_utf8 (self . remaining) { (None , 0) => return None , (None , _) | (Some (_) , 1) => { let byte = self . remaining [0] ; self . remaining = & self . remaining [1 ..] ; byte } (Some (ch) , size) => { self . remaining = & self . remaining [size ..] ; return Some (ch) ; } } ; self . state = match byte { 0x21 ..= 0x5B | 0x5D ..= 0x7E => { return Some (char :: from (byte)) } b'\0' => SpecialEscape ('0') , b'\n' => SpecialEscape ('n') , b'\r' => SpecialEscape ('r') , b'\t' => SpecialEscape ('t') , b'\\' => SpecialEscape ('\\') , _ => HexEscapeX (byte) , } ; Some ('\\') } SpecialEscape (ch) => { self . state = Start ; Some (ch) } HexEscapeX (byte) => { self . state = HexEscapeHighNybble (byte) ; Some ('x') } HexEscapeHighNybble (byte) => { self . state = HexEscapeLowNybble (byte) ; let nybble = byte >> 4 ; Some (hexdigit_to_char (nybble)) } HexEscapeLowNybble (byte) => { self . state = Start ; let nybble = byte & 0xF ; Some (hexdigit_to_char (nybble)) } } } }
    };
}

impl_47!()