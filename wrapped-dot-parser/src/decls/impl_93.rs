macro_rules! deps {
    () => {
        AList!();
        NodeStmt!();
        Node!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < A > From < NodeStmt < A > > for Node < A > { fn from (stmt : NodeStmt < A >) -> Self { Node { id : stmt . node . id . to_string () , port : stmt . node . port , attr : stmt . attr . map (| list | list . into ()) . unwrap_or (AList :: empty ()) , } } }
    };
}

impl_93!();