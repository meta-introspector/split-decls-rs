macro_rules! deps {
    () => {
        BackendTypes!();
    };
}

macro_rules! AbiBuilderMethods {
    () => {
        deps!();
        pub trait AbiBuilderMethods : BackendTypes { fn get_param (& mut self , index : usize) -> Self :: Value ; }
    };
}

AbiBuilderMethods!()