macro_rules! deps {
    () => {
        Edges!();
        IndexType!();
        EdgeType!();
        DefaultIx!();
        NodeIndex!();
    };
}

macro_rules! EdgesConnecting {
    () => {
        deps!();
        # [doc = " Iterator over the multiple directed edges connecting a source node to a target node"] # [derive (Debug , Clone)] pub struct EdgesConnecting < 'a , E : 'a , Ty , Ix : 'a = DefaultIx > where Ty : EdgeType , Ix : IndexType , { target_node : NodeIndex < Ix > , edges : Edges < 'a , E , Ty , Ix > , ty : PhantomData < Ty > , }
    };
}

EdgesConnecting!();