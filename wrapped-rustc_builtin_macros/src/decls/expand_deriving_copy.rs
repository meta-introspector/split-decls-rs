macro_rules! deps {
    () => {
        TraitDef!();
    };
}

macro_rules! expand_deriving_copy {
    () => {
        deps!();
        pub (crate) fn expand_deriving_copy (cx : & ExtCtxt < '_ > , span : Span , mitem : & MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , is_const : bool ,) { let trait_def = TraitDef { span , path : path_std ! (marker :: Copy) , skip_path_as_bound : false , needs_copy_as_bound_if_packed : false , additional_bounds : Vec :: new () , supports_unions : true , methods : Vec :: new () , associated_types : Vec :: new () , is_const , is_staged_api_crate : cx . ecfg . features . staged_api () , } ; trait_def . expand (cx , mitem , item , push) ; }
    };
}

expand_deriving_copy!();