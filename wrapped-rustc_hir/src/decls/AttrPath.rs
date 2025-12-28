macro_rules! AttrPath {
    () => {
        # [derive (Clone , Debug , HashStable_Generic , Encodable , Decodable)] pub struct AttrPath { pub segments : Box < [Ident] > , pub span : Span , }
    };
}

AttrPath!()