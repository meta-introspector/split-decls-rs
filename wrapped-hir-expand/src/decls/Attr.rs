macro_rules! deps {
    () => {
        ModPath!();
        AttrId!();
        AttrInput!();
    };
}

macro_rules! Attr {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct Attr { pub id : AttrId , pub path : Interned < ModPath > , pub input : Option < Box < AttrInput > > , pub ctxt : SyntaxContext , }
    };
}

Attr!()