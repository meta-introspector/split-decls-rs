macro_rules! deps {
    () => {
        Symbol!();
        SymbolName!();
        BytesOrWideString!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Symbol { # [doc = " Returns the name of this function."] # [doc = ""] # [doc = " The returned structure can be used to query various properties about the"] # [doc = " symbol name:"] # [doc = ""] # [doc = " * The `Display` implementation will print out the demangled symbol."] # [doc = " * The raw `str` value of the symbol can be accessed (if it's valid"] # [doc = "   utf-8)."] # [doc = " * The raw bytes for the symbol name can be accessed."] pub fn name (& self) -> Option < SymbolName < '_ > > { self . inner . name () } # [doc = " Returns the starting address of this function."] pub fn addr (& self) -> Option < * mut c_void > { self . inner . addr () } # [doc = " Returns the raw filename as a slice. This is mainly useful for `no_std`"] # [doc = " environments."] pub fn filename_raw (& self) -> Option < BytesOrWideString < '_ > > { self . inner . filename_raw () } # [doc = " Returns the column number for where this symbol is currently executing."] # [doc = ""] # [doc = " Only gimli currently provides a value here and even then only if `filename`"] # [doc = " returns `Some`, and so it is then consequently subject to similar caveats."] pub fn colno (& self) -> Option < u32 > { self . inner . colno () } # [doc = " Returns the line number for where this symbol is currently executing."] # [doc = ""] # [doc = " This return value is typically `Some` if `filename` returns `Some`, and"] # [doc = " is consequently subject to similar caveats."] pub fn lineno (& self) -> Option < u32 > { self . inner . lineno () } # [doc = " Returns the file name where this function was defined."] # [doc = ""] # [doc = " This is currently only available when libbacktrace or gimli is being"] # [doc = " used (e.g. unix platforms other) and when a binary is compiled with"] # [doc = " debuginfo. If neither of these conditions is met then this will likely"] # [doc = " return `None`."] # [doc = ""] # [doc = " # Required features"] # [doc = ""] # [doc = " This function requires the `std` feature of the `backtrace` crate to be"] # [doc = " enabled, and the `std` feature is enabled by default."] # [cfg (feature = "std")] # [allow (unreachable_code)] pub fn filename (& self) -> Option < & Path > { self . inner . filename () } }
    };
}

impl_19!()