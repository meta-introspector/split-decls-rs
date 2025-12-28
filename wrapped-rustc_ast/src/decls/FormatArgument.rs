macro_rules! deps {
    () => {
        Walkable!();
        Expr!();
        FormatArgumentKind!();
    };
}

macro_rules! FormatArgument {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct FormatArgument { pub kind : FormatArgumentKind , pub expr : Box < Expr > , }
    };
}

FormatArgument!();