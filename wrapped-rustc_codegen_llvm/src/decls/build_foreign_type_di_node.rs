macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! build_foreign_type_di_node {
    () => {
        deps!();
        fn build_foreign_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , t : Ty < 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { debug ! ("build_foreign_type_di_node: {:?}" , t) ; let & ty :: Foreign (def_id) = unique_type_id . expect_ty () . kind () else { bug ! ("build_foreign_type_di_node() called with unexpected type: {:?}" , unique_type_id . expect_ty ()) ; } ; build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & compute_debuginfo_type_name (cx . tcx , t , false) , None , cx . size_and_align_of (t) , Some (get_namespace_for_item (cx , def_id)) , DIFlags :: FlagZero ,) , | _ , _ | smallvec ! [] , NO_GENERICS ,) }
    };
}

build_foreign_type_di_node!()