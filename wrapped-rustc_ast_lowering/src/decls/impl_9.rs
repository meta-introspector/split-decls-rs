macro_rules! deps {
    () => {
        SelfResolver!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'ast , 'a > Visitor < 'ast > for SelfResolver < 'a > { fn visit_id (& mut self , id : NodeId) { self . try_replace_id (id) ; } }
    };
}

impl_9!();