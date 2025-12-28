macro_rules! deps {
    () => {
        FormatOptions!();
        FormatArgPosition!();
        Walkable!();
        FormatTrait!();
    };
}

macro_rules! FormatPlaceholder {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq , Walkable)] pub struct FormatPlaceholder { # [doc = " Index into [`FormatArgs::arguments`]."] pub argument : FormatArgPosition , # [doc = " The span inside the format string for the full `{…}` placeholder."] pub span : Option < Span > , # [doc = " `{}`, `{:?}`, or `{:x}`, etc."] # [visitable (ignore)] pub format_trait : FormatTrait , # [doc = " `{}` or `{:.5}` or `{:-^20}`, etc."] # [visitable (ignore)] pub format_options : FormatOptions , }
    };
}

FormatPlaceholder!();