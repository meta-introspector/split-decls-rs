macro_rules! deps {
    () => {
        StructureNodeKind!();
    };
}

macro_rules! StructureNode {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct StructureNode { pub parent : Option < usize > , pub label : String , pub navigation_range : TextRange , pub node_range : TextRange , pub kind : StructureNodeKind , pub detail : Option < String > , pub deprecated : bool , }
    };
}

StructureNode!();