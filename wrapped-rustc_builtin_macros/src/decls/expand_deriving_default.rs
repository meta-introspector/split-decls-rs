macro_rules! deps {
    () => {
        TraitDef!();
        DetectNonVariantDefaultAttr!();
        Bounds!();
        FieldlessVariantsStrategy!();
        MethodDef!();
        Path!();
    };
}

macro_rules! expand_deriving_default {
    () => {
        deps!();
        pub (crate) fn expand_deriving_default (cx : & ExtCtxt < '_ > , span : Span , mitem : & ast :: MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , is_const : bool ,) { item . visit_with (& mut DetectNonVariantDefaultAttr { cx }) ; let trait_def = TraitDef { span , path : Path :: new (vec ! [kw :: Default , sym :: Default]) , skip_path_as_bound : has_a_default_variant (item) , needs_copy_as_bound_if_packed : false , additional_bounds : Vec :: new () , supports_unions : false , methods : vec ! [MethodDef { name : kw :: Default , generics : Bounds :: empty () , explicit_self : false , nonself_args : Vec :: new () , ret_ty : Self_ , attributes : thin_vec ! [cx . attr_word (sym :: inline , span)] , fieldless_variants_strategy : FieldlessVariantsStrategy :: Default , combine_substructure : combine_substructure (Box :: new (| cx , trait_span , substr | { match substr . fields { StaticStruct (_ , fields) => { default_struct_substructure (cx , trait_span , substr , fields) } StaticEnum (enum_def) => { default_enum_substructure (cx , trait_span , enum_def , item . span ()) } _ => cx . dcx () . span_bug (trait_span , "method in `derive(Default)`") , } })) , }] , associated_types : Vec :: new () , is_const , is_staged_api_crate : cx . ecfg . features . staged_api () , } ; trait_def . expand (cx , mitem , item , push) }
    };
}

expand_deriving_default!();