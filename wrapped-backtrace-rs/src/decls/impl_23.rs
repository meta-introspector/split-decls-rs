macro_rules! deps {
    () => {
        SymbolName!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'a > SymbolName < 'a > { # [doc = " Creates a new symbol name from the raw underlying bytes."] pub fn new (bytes : & 'a [u8]) -> SymbolName < 'a > { let str_bytes = str :: from_utf8 (bytes) . ok () ; let demangled = str_bytes . and_then (| s | try_demangle (s) . ok ()) ; # [cfg (feature = "cpp_demangle")] let cpp = if demangled . is_none () { OptionCppSymbol :: parse (bytes) } else { OptionCppSymbol :: none () } ; SymbolName { bytes , demangled , # [cfg (feature = "cpp_demangle")] cpp_demangled : cpp , } } # [doc = " Returns the raw (mangled) symbol name as a `str` if the symbol is valid utf-8."] # [doc = ""] # [doc = " Use the `Display` implementation if you want the demangled version."] pub fn as_str (& self) -> Option < & 'a str > { self . demangled . as_ref () . map (| s | s . as_str ()) . or_else (| | str :: from_utf8 (self . bytes) . ok ()) } # [doc = " Returns the raw symbol name as a list of bytes"] pub fn as_bytes (& self) -> & 'a [u8] { self . bytes } }
    };
}

impl_23!()