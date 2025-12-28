macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! build_cpp_f16_di_node {
    () => {
        deps!();
        fn build_cpp_f16_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx >) -> DINodeCreationResult < 'll > { let float_ty = cx . tcx . types . f16 ; let bits_ty = cx . tcx . types . u16 ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { match float_ty . kind () { ty :: Adt (def , _) => Some (file_metadata_from_def_id (cx , Some (def . did ()))) , _ => None , } } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , UniqueTypeId :: for_ty (cx . tcx , float_ty) , "f16" , def_location , cx . size_and_align_of (float_ty) , NO_SCOPE_METADATA , DIFlags :: FlagZero ,) , | cx , float_di_node | { let def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { match bits_ty . kind () { ty :: Adt (def , _) => Some (def . did ()) , _ => None , } } else { None } ; smallvec ! [build_field_di_node (cx , float_di_node , "bits" , cx . layout_of (bits_ty) , Size :: ZERO , DIFlags :: FlagZero , type_di_node (cx , bits_ty) , def_id ,)] } , NO_GENERICS ,) }
    };
}

build_cpp_f16_di_node!()