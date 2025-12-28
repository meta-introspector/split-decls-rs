macro_rules! deps {
    () => {
        SynInfoTrait!();
        SynDetails!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl SynInfoTrait for SynDetails { fn parsed_type (& self) -> Option < & str > { match self { SynDetails :: Info (info) => Some (& info . parsed_type) , _ => None , } } fn version (& self) -> Option < & str > { match self { SynDetails :: Info (info) => Some (& info . version) , _ => None , } } }
    };
}

impl_47!()