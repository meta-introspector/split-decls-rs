macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! uncached_llvm_type {
    () => {
        deps!();
        fn uncached_llvm_type < 'a , 'tcx > (cx : & CodegenCx < 'a , 'tcx > , layout : TyAndLayout < 'tcx > , defer : & mut Option < (& 'a Type , TyAndLayout < 'tcx >) > ,) -> & 'a Type { match layout . backend_repr { BackendRepr :: Scalar (_) => bug ! ("handled elsewhere") , BackendRepr :: SimdVector { element , count } => { let element = layout . scalar_llvm_type_at (cx , element) ; return cx . type_vector (element , count) ; } BackendRepr :: Memory { .. } | BackendRepr :: ScalarPair (..) => { } } let name = match layout . ty . kind () { ty :: Adt (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Foreign (..) | ty :: Coroutine (..) | ty :: Str if ! cx . sess () . fewer_names () => { let mut name = with_no_visible_paths ! (with_no_trimmed_paths ! (layout . ty . to_string ())) ; if let (& ty :: Adt (def , _) , & Variants :: Single { index }) = (layout . ty . kind () , & layout . variants) { if def . is_enum () { write ! (& mut name , "::{}" , def . variant (index) . name) . unwrap () ; } } if let (& ty :: Coroutine (_ , _) , & Variants :: Single { index }) = (layout . ty . kind () , & layout . variants) { write ! (& mut name , "::{}" , ty :: CoroutineArgs :: variant_name (index)) . unwrap () ; } Some (name) } _ => None , } ; match layout . fields { FieldsShape :: Primitive | FieldsShape :: Union (_) => { let fill = cx . type_padding_filler (layout . size , layout . align . abi) ; let packed = false ; match name { None => cx . type_struct (& [fill] , packed) , Some (ref name) => { let llty = cx . type_named_struct (name) ; cx . set_struct_body (llty , & [fill] , packed) ; llty } } } FieldsShape :: Array { count , .. } => cx . type_array (layout . field (cx , 0) . llvm_type (cx) , count) , FieldsShape :: Arbitrary { .. } => match name { None => { let (llfields , packed) = struct_llfields (cx , layout) ; cx . type_struct (& llfields , packed) } Some (ref name) => { let llty = cx . type_named_struct (name) ; * defer = Some ((llty , layout)) ; llty } } , } }
    };
}

uncached_llvm_type!()