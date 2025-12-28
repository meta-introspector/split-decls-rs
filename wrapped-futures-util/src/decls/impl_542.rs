macro_rules! deps {
    () => {
        FlowController!();
    };
}

macro_rules! impl_542 {
    () => {
        deps!();
        impl < St , Fc > FusedStream for FlattenUnorderedWithFlowController < St , Fc > where St : FusedStream , Fc : FlowController < St :: Item , < St :: Item as Stream > :: Item > , St :: Item : Stream + Unpin , { fn is_terminated (& self) -> bool { self . stream . is_terminated () && self . inner_streams . is_empty () } }
    };
}

impl_542!()