// Generated macro for demangle (function)
macro_rules! Depcrate_v0demangle {
() => {
// Module: crate::v0
// Provides: {"demangle"}
// Dependencies: {}
# [doc = " De-mangles a Rust symbol into a more readable version"] # [doc = ""] # [doc = " This function will take a **mangled** symbol and return a value. When printed,"] # [doc = " the de-mangled version will be written. If the symbol does not look like"] # [doc = " a mangled symbol, the original value will be written instead."] pub fn demangle (s : & str) -> Result < (Demangle , & str) , ParseError > { let inner ; if s . len () > 2 && s . starts_with ("_R") { inner = & s [2 ..] ; } else if s . len () > 1 && s . starts_with ('R') { inner = & s [1 ..] ; } else if s . len () > 3 && s . starts_with ("__R") { inner = & s [3 ..] ; } else { return Err (ParseError :: Invalid) ; } match inner . as_bytes () [0] { b'A' ..= b'Z' => { } _ => return Err (ParseError :: Invalid) , } if inner . bytes () . any (| c | c & 0x80 != 0) { return Err (ParseError :: Invalid) ; } let try_parse_path = | parser | { let mut dummy_printer = Printer { parser : Ok (parser) , out : None , bound_lifetime_depth : 0 , } ; dummy_printer . print_path (false) . expect ("`fmt::Error`s should be impossible without a `fmt::Formatter`") ; dummy_printer . parser } ; let mut parser = Parser { sym : inner , next : 0 , depth : 0 , } ; parser = try_parse_path (parser) ? ; if let Some (& (b'A' ..= b'Z')) = parser . sym . as_bytes () . get (parser . next) { parser = try_parse_path (parser) ? ; } Ok ((Demangle { inner } , & parser . sym [parser . next ..])) }
};
}
