macro_rules! deps {
    () => {
        CodegenCx!();
        DIB!();
        Bool!();
    };
}

macro_rules! build_basic_type_di_node {
    () => {
        deps!();
        fn build_basic_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , t : Ty < 'tcx > ,) -> DINodeCreationResult < 'll > { debug ! ("build_basic_type_di_node: {:?}" , t) ; let cpp_like_debuginfo = cpp_like_debuginfo (cx . tcx) ; use dwarf_const :: { DW_ATE_UTF , DW_ATE_boolean , DW_ATE_float , DW_ATE_signed , DW_ATE_unsigned } ; let (name , encoding) = match t . kind () { ty :: Never => ("!" , DW_ATE_unsigned) , ty :: Tuple (elements) if elements . is_empty () => { if cpp_like_debuginfo { return build_tuple_type_di_node (cx , UniqueTypeId :: for_ty (cx . tcx , t)) ; } else { ("()" , DW_ATE_unsigned) } } ty :: Bool => ("bool" , DW_ATE_boolean) , ty :: Char => ("char" , DW_ATE_UTF) , ty :: Int (int_ty) if cpp_like_debuginfo => (int_ty . msvc_basic_name () , DW_ATE_signed) , ty :: Uint (uint_ty) if cpp_like_debuginfo => (uint_ty . msvc_basic_name () , DW_ATE_unsigned) , ty :: Float (ty :: FloatTy :: F16) if cpp_like_debuginfo => { return build_cpp_f16_di_node (cx) ; } ty :: Float (float_ty) if cpp_like_debuginfo => (float_ty . msvc_basic_name () , DW_ATE_float) , ty :: Int (int_ty) => (int_ty . name_str () , DW_ATE_signed) , ty :: Uint (uint_ty) => (uint_ty . name_str () , DW_ATE_unsigned) , ty :: Float (float_ty) => (float_ty . name_str () , DW_ATE_float) , _ => bug ! ("debuginfo::build_basic_type_di_node - `t` is invalid type") , } ; let ty_di_node = create_basic_type (cx , name , cx . size_of (t) , encoding) ; if ! cpp_like_debuginfo { return DINodeCreationResult :: new (ty_di_node , false) ; } let typedef_name = match t . kind () { ty :: Int (int_ty) => int_ty . name_str () , ty :: Uint (uint_ty) => uint_ty . name_str () , ty :: Float (float_ty) => float_ty . name_str () , _ => return DINodeCreationResult :: new (ty_di_node , false) , } ; let typedef_di_node = unsafe { llvm :: LLVMRustDIBuilderCreateTypedef (DIB (cx) , ty_di_node , typedef_name . as_c_char_ptr () , typedef_name . len () , unknown_file_metadata (cx) , 0 , None ,) } ; DINodeCreationResult :: new (typedef_di_node , false) }
    };
}

build_basic_type_di_node!();