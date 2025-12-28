macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
        PathSegment!();
        Walkable!();
    };
}

macro_rules! Path {
    () => {
        deps!();
        # [doc = " A \"Path\" is essentially Rust's notion of a name."] # [doc = ""] # [doc = " It's represented as a sequence of identifiers,"] # [doc = " along with a bunch of supporting information."] # [doc = ""] # [doc = " E.g., `std::cmp::PartialEq`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Path { pub span : Span , # [doc = " The segments in the path: the things separated by `::`."] # [doc = " Global paths begin with `kw::PathRoot`."] pub segments : ThinVec < PathSegment > , pub tokens : Option < LazyAttrTokenStream > , }
    };
}

Path!();