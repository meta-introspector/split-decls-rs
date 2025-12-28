macro_rules! deps {
    () => {
        MethodDef!();
        Bounds!();
        TraitDef!();
        FieldlessVariantsStrategy!();
        Path!();
    };
}

macro_rules! expand_deriving_debug {
    () => {
        deps!();
        pub (crate) fn expand_deriving_debug (cx : & ExtCtxt < '_ > , span : Span , mitem : & MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , is_const : bool ,) { let fmtr = Ref (Box :: new (Path (path_std ! (fmt :: Formatter))) , ast :: Mutability :: Mut) ; let trait_def = TraitDef { span , path : path_std ! (fmt :: Debug) , skip_path_as_bound : false , needs_copy_as_bound_if_packed : true , additional_bounds : Vec :: new () , supports_unions : false , methods : vec ! [MethodDef { name : sym :: fmt , generics : Bounds :: empty () , explicit_self : true , nonself_args : vec ! [(fmtr , sym :: f)] , ret_ty : Path (path_std ! (fmt :: Result)) , attributes : thin_vec ! [cx . attr_word (sym :: inline , span)] , fieldless_variants_strategy : FieldlessVariantsStrategy :: SpecializeIfAllVariantsFieldless , combine_substructure : combine_substructure (Box :: new (| a , b , c | { show_substructure (a , b , c) })) , }] , associated_types : Vec :: new () , is_const , is_staged_api_crate : cx . ecfg . features . staged_api () , } ; trait_def . expand (cx , mitem , item , push) }
    };
}

expand_deriving_debug!();