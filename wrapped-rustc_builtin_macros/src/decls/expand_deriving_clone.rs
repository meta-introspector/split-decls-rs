macro_rules! deps {
    () => {
        MethodDef!();
        FieldlessVariantsStrategy!();
        TraitDef!();
        Path!();
        Bounds!();
    };
}

macro_rules! expand_deriving_clone {
    () => {
        deps!();
        pub (crate) fn expand_deriving_clone (cx : & ExtCtxt < '_ > , span : Span , mitem : & MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , is_const : bool ,) { let bounds ; let substructure ; let is_simple ; match item { Annotatable :: Item (annitem) => match & annitem . kind { ItemKind :: Struct (_ , Generics { params , .. } , _) | ItemKind :: Enum (_ , Generics { params , .. } , _) => { let container_id = cx . current_expansion . id . expn_data () . parent . expect_local () ; let has_derive_copy = cx . resolver . has_derive_copy (container_id) ; if has_derive_copy && ! params . iter () . any (| param | matches ! (param . kind , ast :: GenericParamKind :: Type { .. })) { bounds = vec ! [] ; is_simple = true ; substructure = combine_substructure (Box :: new (| c , s , sub | { cs_clone_simple ("Clone" , c , s , sub , false) })) ; } else { bounds = vec ! [] ; is_simple = false ; substructure = combine_substructure (Box :: new (| c , s , sub | cs_clone ("Clone" , c , s , sub))) ; } } ItemKind :: Union (..) => { bounds = vec ! [Path (path_std ! (marker :: Copy))] ; is_simple = true ; substructure = combine_substructure (Box :: new (| c , s , sub | { cs_clone_simple ("Clone" , c , s , sub , true) })) ; } _ => cx . dcx () . span_bug (span , "`#[derive(Clone)]` on wrong item kind") , } , _ => cx . dcx () . span_bug (span , "`#[derive(Clone)]` on trait item or impl item") , } let trait_def = TraitDef { span , path : path_std ! (clone :: Clone) , skip_path_as_bound : false , needs_copy_as_bound_if_packed : true , additional_bounds : bounds , supports_unions : true , methods : vec ! [MethodDef { name : sym :: clone , generics : Bounds :: empty () , explicit_self : true , nonself_args : Vec :: new () , ret_ty : Self_ , attributes : thin_vec ! [cx . attr_word (sym :: inline , span)] , fieldless_variants_strategy : FieldlessVariantsStrategy :: Default , combine_substructure : substructure , }] , associated_types : Vec :: new () , is_const , is_staged_api_crate : cx . ecfg . features . staged_api () , } ; trait_def . expand_ext (cx , mitem , item , push , is_simple) }
    };
}

expand_deriving_clone!();