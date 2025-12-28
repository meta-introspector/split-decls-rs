macro_rules! deps {
    () => {
        AList!();
        NodeID!();
        Node!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < A > From < & NodeID > for Node < A > { fn from (node : & NodeID) -> Self { Node { id : node . id . to_string () , port : node . port . clone () , attr : AList :: empty () , } } }
    };
}

impl_94!()