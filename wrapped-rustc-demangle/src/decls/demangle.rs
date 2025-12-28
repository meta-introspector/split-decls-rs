macro_rules! deps {
    () => {
        ParseError!();
        DemangleStyle!();
        Demangle!();
    };
}

macro_rules! demangle {
    () => {
        deps!();
        # [doc = " De-mangles a Rust symbol into a more readable version"] # [doc = ""] # [doc = " This function will take a **mangled** symbol and return a value. When printed,"] # [doc = " the de-mangled version will be written. If the symbol does not look like"] # [doc = " a mangled symbol, the original value will be written instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rustc_demangle::demangle;"] # [doc = ""] # [doc = " assert_eq!(demangle(\"_ZN4testE\").to_string(), \"test\");"] # [doc = " assert_eq!(demangle(\"_ZN3foo3barE\").to_string(), \"foo::bar\");"] # [doc = " assert_eq!(demangle(\"foo\").to_string(), \"foo\");"] # [doc = " ```"] pub fn demangle (mut s : & str) -> Demangle { let llvm = ".llvm." ; if let Some (i) = s . find (llvm) { let candidate = & s [i + llvm . len () ..] ; let all_hex = candidate . chars () . all (| c | match c { 'A' ..= 'F' | '0' ..= '9' | '@' => true , _ => false , }) ; if all_hex { s = & s [.. i] ; } } let mut suffix = "" ; let mut style = match legacy :: demangle (s) { Ok ((d , s)) => { suffix = s ; Some (DemangleStyle :: Legacy (d)) } Err (()) => match v0 :: demangle (s) { Ok ((d , s)) => { suffix = s ; Some (DemangleStyle :: V0 (d)) } Err (v0 :: ParseError :: Invalid) | Err (v0 :: ParseError :: RecursedTooDeep) => None , } , } ; if ! suffix . is_empty () { if suffix . starts_with ('.') && is_symbol_like (suffix) { } else { suffix = "" ; style = None ; } } Demangle { style , original : s , suffix , } }
    };
}

demangle!();