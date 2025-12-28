macro_rules! deps {
    () => {
        BackendTypes!();
    };
}

macro_rules! StaticBuilderMethods {
    () => {
        deps!();
        pub trait StaticBuilderMethods : BackendTypes { fn get_static (& mut self , def_id : DefId) -> Self :: Value ; }
    };
}

StaticBuilderMethods!();