macro_rules! deps {
    () => {
        AstNodeWrapper!();
        HasNodeId!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < Wrapped : HasNodeId , Tag > HasNodeId for AstNodeWrapper < Wrapped , Tag > { fn node_id (& self) -> NodeId { self . wrapped . node_id () } fn node_id_mut (& mut self) -> & mut NodeId { self . wrapped . node_id_mut () } }
    };
}

impl_267!();