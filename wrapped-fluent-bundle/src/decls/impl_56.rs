macro_rules! deps {
    () => {
        FluentResource!();
        ResolveValue!();
        FluentValue!();
        MemoizerKind!();
        Scope!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'bundle > ResolveValue < 'bundle > for ast :: Pattern < & 'bundle str > { fn resolve < 'ast , 'args , 'errors , R , M > (& 'ast self , scope : & mut Scope < 'bundle , 'ast , 'args , 'errors , R , M > ,) -> FluentValue < 'bundle > where R : Borrow < FluentResource > , M : MemoizerKind , { let len = self . elements . len () ; if len == 1 { if let ast :: PatternElement :: TextElement { value } = self . elements [0] { return scope . bundle . transform . map_or_else (| | value . into () , | transform | transform (value) . into ()) ; } } let mut result = String :: new () ; self . write (& mut result , scope) . expect ("Failed to write to a string.") ; result . into () } }
    };
}

impl_56!();