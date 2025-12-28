macro_rules! deps {
    () => {
        Expander!();
        CfgAccessibleIndeterminate!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl MultiItemModifier for Expander { fn expand (& self , ecx : & mut ExtCtxt < '_ > , span : Span , meta_item : & ast :: MetaItem , item : Annotatable , _is_derive_const : bool ,) -> ExpandResult < Vec < Annotatable > , Annotatable > { let template = AttributeTemplate { list : Some (& ["path"]) , .. Default :: default () } ; validate_attr :: check_builtin_meta_item (& ecx . sess . psess , meta_item , ast :: AttrStyle :: Outer , sym :: cfg_accessible , template , true ,) ; let Some (path) = validate_input (ecx , meta_item) else { return ExpandResult :: Ready (Vec :: new ()) ; } ; match ecx . resolver . cfg_accessible (ecx . current_expansion . id , path) { Ok (true) => ExpandResult :: Ready (vec ! [item]) , Ok (false) => ExpandResult :: Ready (Vec :: new ()) , Err (Indeterminate) if ecx . force_mode => { ecx . dcx () . emit_err (errors :: CfgAccessibleIndeterminate { span }) ; ExpandResult :: Ready (vec ! [item]) } Err (Indeterminate) => ExpandResult :: Retry (item) , } } }
    };
}

impl_17!();