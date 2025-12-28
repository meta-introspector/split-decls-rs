macro_rules! SelfResolver {
    () => {
        struct SelfResolver < 'a > { resolver : & 'a mut ResolverAstLowering , path_id : NodeId , self_param_id : NodeId , }
    };
}

SelfResolver!()