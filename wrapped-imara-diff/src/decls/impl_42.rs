macro_rules! deps {
    () => {
        IndentLevel!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl IndentLevel { # [doc = " line is empty or only contains whitespaces (or EOF)"] const BLANK : IndentLevel = IndentLevel (u8 :: MAX) ; const MAX : IndentLevel = IndentLevel (200) ; pub fn for_ascii_line (src : impl IntoIterator < Item = u8 > , tab_width : u8) -> IndentLevel { let mut indent_level = IndentLevel (0) ; for c in src { match c { b' ' => indent_level . 0 += 1 , b'\t' => indent_level . 0 += tab_width - indent_level . 0 % tab_width , b'\r' | b'\n' | b'\x0C' => () , _ => return indent_level , } if indent_level >= Self :: MAX { return indent_level ; } } IndentLevel :: BLANK } pub fn for_line (src : impl IntoIterator < Item = char > , tab_width : u8) -> IndentLevel { let mut indent_level = IndentLevel (0) ; for c in src { match c { ' ' => indent_level . 0 += 1 , '\t' => indent_level . 0 += tab_width - indent_level . 0 % tab_width , '\r' | '\n' | '\x0C' => () , _ => return indent_level , } if indent_level >= Self :: MAX { return indent_level ; } } IndentLevel :: BLANK } fn map_or < T > (self , default : T , f : impl FnOnce (u8) -> T) -> T { if self == Self :: BLANK { default } else { f (self . 0) } } fn or (self , default : Self) -> Self { if self == Self :: BLANK { default } else { self } } }
    };
}

impl_42!()