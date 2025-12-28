macro_rules! deps {
    () => {
        LitKind!();
        Walkable!();
        FormatArgsPiece!();
        FormatArguments!();
    };
}

macro_rules! FormatArgs {
    () => {
        deps!();
        # [doc = " (Parsed) format args."] # [doc = ""] # [doc = " Basically the \"AST\" for a complete `format_args!()`."] # [doc = ""] # [doc = " E.g., `format_args!(\"hello {name}\");`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct FormatArgs { pub span : Span , pub template : Vec < FormatArgsPiece > , pub arguments : FormatArguments , # [doc = " The raw, un-split format string literal, with no escaping or processing."] # [doc = ""] # [doc = " Generally only useful for lints that care about the raw bytes the user wrote."] pub uncooked_fmt_str : (LitKind , Symbol) , # [doc = " Was the format literal written in the source?"] # [doc = " - `format!(\"boo\")` => true,"] # [doc = " - `format!(concat!(\"b\", \"o\", \"o\"))` => false,"] # [doc = " - `format!(include_str!(\"boo.txt\"))` => false,"] # [doc = ""] # [doc = " If it wasn't written in the source then we have to be careful with spans pointing into it"] # [doc = " and suggestions about rewriting it."] pub is_source_literal : bool , }
    };
}

FormatArgs!();