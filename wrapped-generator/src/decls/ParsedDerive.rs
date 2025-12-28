macro_rules! ParsedDerive {
    () => {
        # [doc = " Parsed information of the derive and the attributes."] pub struct ParsedDerive { # [doc = " The identifier of the deriving struct, union, or enum."] pub name : Ident , # [doc = " The generics of the deriving struct, union, or enum."] pub generics : Generics , # [doc = " Indicates whether the 'non_exhaustive' attribute is added to the 'Rule' enum."] pub non_exhaustive : bool , }
    };
}

ParsedDerive!();