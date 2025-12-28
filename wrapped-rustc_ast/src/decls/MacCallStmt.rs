macro_rules! deps {
    () => {
        Walkable!();
        AttrVec!();
        MacCall!();
        MacStmtStyle!();
        LazyAttrTokenStream!();
    };
}

macro_rules! MacCallStmt {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct MacCallStmt { pub mac : Box < MacCall > , pub style : MacStmtStyle , pub attrs : AttrVec , pub tokens : Option < LazyAttrTokenStream > , }
    };
}

MacCallStmt!();