macro_rules! deps {
    () => {
        FormatArgumentKind!();
        Expr!();
        Walkable!();
    };
}

macro_rules! FormatArgument {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct FormatArgument { pub kind : FormatArgumentKind , pub expr : Box < Expr > , }
    };
}

FormatArgument!()