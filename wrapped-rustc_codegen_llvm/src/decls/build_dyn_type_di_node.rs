macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! build_dyn_type_di_node {
    () => {
        deps!();
        # [doc = " Create debuginfo for `dyn SomeTrait` types. Currently these are empty structs"] # [doc = " we with the correct type name (e.g. \"dyn SomeTrait<Foo, Item=u32> + Sync\")."] fn build_dyn_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , dyn_type : Ty < 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { if let ty :: Dynamic (..) = dyn_type . kind () { let type_name = compute_debuginfo_type_name (cx . tcx , dyn_type , true) ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & type_name , None , cx . size_and_align_of (dyn_type) , NO_SCOPE_METADATA , DIFlags :: FlagZero ,) , | _ , _ | smallvec ! [] , NO_GENERICS ,) } else { bug ! ("Only ty::Dynamic is valid for build_dyn_type_di_node(). Found {:?} instead." , dyn_type) } }
    };
}

build_dyn_type_di_node!()