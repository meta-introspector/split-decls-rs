macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl Arbitrary for char { fn arbitrary (g : & mut Gen) -> char { let mode = g . random_range (0 .. 100) ; match mode { 0 ..= 49 => { g . random_range (0u8 .. 0xB0) as char } 50 ..= 59 => { loop { if let Some (x) = char :: from_u32 (g . random_range (0 .. 0x10000)) { return x ; } } } 60 ..= 84 => { g . choose (& [' ' , ' ' , ' ' , '\t' , '\n' , '~' , '`' , '!' , '@' , '#' , '$' , '%' , '^' , '&' , '*' , '(' , ')' , '_' , '-' , '=' , '+' , '[' , ']' , '{' , '}' , ':' , ';' , '\'' , '"' , '\\' , '|' , ',' , '<' , '>' , '.' , '/' , '?' , '0' , '1' , '2' , '3' , '4' , '5' , '6' , '7' , '8' , '9' ,]) . unwrap () . to_owned () } 85 ..= 89 => { g . choose (& ['\u{0149}' , '\u{fff0}' , '\u{fff1}' , '\u{fff2}' , '\u{fff3}' , '\u{fff4}' , '\u{fff5}' , '\u{fff6}' , '\u{fff7}' , '\u{fff8}' , '\u{fff9}' , '\u{fffA}' , '\u{fffB}' , '\u{fffC}' , '\u{fffD}' , '\u{fffE}' , '\u{fffF}' , '\u{0600}' , '\u{0601}' , '\u{0602}' , '\u{0603}' , '\u{0604}' , '\u{0605}' , '\u{061C}' , '\u{06DD}' , '\u{070F}' , '\u{180E}' , '\u{110BD}' , '\u{1D173}' , '\u{e0001}' , '\u{e0020}' , '\u{e000}' , '\u{e001}' , '\u{ef8ff}' , '\u{f0000}' , '\u{ffffd}' , '\u{ffffe}' , '\u{fffff}' , '\u{100000}' , '\u{10FFFD}' , '\u{10FFFE}' , '\u{10FFFF}' , '\u{3000}' , '\u{1680}' ,]) . unwrap () . to_owned () } 90 ..= 94 => { char :: from_u32 (g . random_range (0x2000 .. 0x2070)) . unwrap () } 95 ..= 99 => { g . random () } _ => unreachable ! () , } } fn shrink (& self) -> Box < dyn Iterator < Item = char > > { Box :: new ((* self as u32) . shrink () . filter_map (char :: from_u32)) } }
    };
}

impl_39!();