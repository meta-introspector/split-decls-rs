macro_rules! deps {
    () => {
        Safety!();
        Path!();
        MetaItemKind!();
    };
}

macro_rules! MetaItem {
    () => {
        deps!();
        # [doc = " A semantic representation of a meta item. A meta item is a slightly"] # [doc = " restricted form of an attribute -- it can only contain expressions in"] # [doc = " certain leaf positions, rather than arbitrary token streams -- that is used"] # [doc = " for most built-in attributes."] # [doc = ""] # [doc = " E.g., `#[test]`, `#[derive(..)]`, `#[rustfmt::skip]` or `#[feature = \"foo\"]`."] # [derive (Clone , Encodable , Decodable , Debug , HashStable_Generic)] pub struct MetaItem { pub unsafety : Safety , pub path : Path , pub kind : MetaItemKind , pub span : Span , }
    };
}

MetaItem!()