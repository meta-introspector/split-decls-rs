macro_rules! deps {
    () => {
        Green!();
        SyntaxElement!();
    };
}

macro_rules! NodeData {
    () => {
        deps!();
        struct NodeData { _c : Count < _SyntaxElement > , rc : Cell < u32 > , parent : Cell < Option < ptr :: NonNull < NodeData > > > , index : Cell < u32 > , green : Green , # [doc = " Invariant: never changes after NodeData is created."] mutable : bool , # [doc = " Absolute offset for immutable nodes, unused for mutable nodes."] offset : TextSize , first : Cell < * const NodeData > , # [doc = " Invariant: never null if mutable."] next : Cell < * const NodeData > , # [doc = " Invariant: never null if mutable."] prev : Cell < * const NodeData > , }
    };
}

NodeData!();