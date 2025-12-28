macro_rules! deps {
    () => {
        HasStaticRootDefId!();
        CompileTimeMachine!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl HasStaticRootDefId for const_eval :: CompileTimeMachine < '_ > { fn static_def_id (& self) -> Option < LocalDefId > { Some (self . static_root_ids ? . 1) } }
    };
}

impl_222!()