macro_rules! deps {
    () => {
        Walkable!();
        AngleBracketedArg!();
    };
}

macro_rules! AngleBracketedArgs {
    () => {
        deps!();
        # [doc = " A path like `Foo<'a, T>`."] # [derive (Clone , Encodable , Decodable , Debug , Default , Walkable)] pub struct AngleBracketedArgs { # [doc = " The overall span."] pub span : Span , # [doc = " The comma separated parts in the `<...>`."] pub args : ThinVec < AngleBracketedArg > , }
    };
}

AngleBracketedArgs!();