macro_rules! deps {
    () => {
        LlvmInfoTrait!();
        LlvmDetails!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl LlvmInfoTrait for LlvmDetails { fn ir_version (& self) -> Option < & str > { match self { LlvmDetails :: Info (info) => Some (& info . ir_version) , _ => None , } } fn target_triple (& self) -> Option < & str > { match self { LlvmDetails :: Info (info) => Some (& info . target_triple) , _ => None , } } }
    };
}

impl_27!();