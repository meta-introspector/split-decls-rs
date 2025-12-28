macro_rules! deps {
    () => {
        Path!();
        FieldlessVariantsStrategy!();
        Bounds!();
        MethodDef!();
        PathKind!();
        TraitDef!();
    };
}

macro_rules! expand_deriving_hash {
    () => {
        deps!();
        pub (crate) fn expand_deriving_hash (cx : & ExtCtxt < '_ > , span : Span , mitem : & MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , is_const : bool ,) { let path = Path :: new_ (pathvec_std ! (hash :: Hash) , vec ! [] , PathKind :: Std) ; let typaram = sym :: __H ; let arg = Path :: new_local (typaram) ; let hash_trait_def = TraitDef { span , path , skip_path_as_bound : false , needs_copy_as_bound_if_packed : true , additional_bounds : Vec :: new () , supports_unions : false , methods : vec ! [MethodDef { name : sym :: hash , generics : Bounds { bounds : vec ! [(typaram , vec ! [path_std ! (hash :: Hasher)])] } , explicit_self : true , nonself_args : vec ! [(Ref (Box :: new (Path (arg)) , Mutability :: Mut) , sym :: state)] , ret_ty : Unit , attributes : thin_vec ! [cx . attr_word (sym :: inline , span)] , fieldless_variants_strategy : FieldlessVariantsStrategy :: Unify , combine_substructure : combine_substructure (Box :: new (| a , b , c | { hash_substructure (a , b , c) })) , }] , associated_types : Vec :: new () , is_const , is_staged_api_crate : cx . ecfg . features . staged_api () , } ; hash_trait_def . expand (cx , mitem , item , push) ; }
    };
}

expand_deriving_hash!();