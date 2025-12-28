macro_rules! deps {
    () => {
        Walkable!();
        Path!();
        UseTreeKind!();
    };
}

macro_rules! UseTree {
    () => {
        deps!();
        # [doc = " A tree of paths sharing common prefixes."] # [doc = " Used in `use` items both at top-level and inside of braces in import groups."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct UseTree { pub prefix : Path , pub kind : UseTreeKind , pub span : Span , }
    };
}

UseTree!()