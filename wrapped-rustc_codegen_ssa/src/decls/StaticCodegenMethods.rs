macro_rules! deps {
    () => {
        BackendTypes!();
    };
}

macro_rules! StaticCodegenMethods {
    () => {
        deps!();
        pub trait StaticCodegenMethods : BackendTypes { fn static_addr_of (& self , cv : Self :: Value , align : Align , kind : Option < & str >) -> Self :: Value ; fn codegen_static (& mut self , def_id : DefId) ; }
    };
}

StaticCodegenMethods!();