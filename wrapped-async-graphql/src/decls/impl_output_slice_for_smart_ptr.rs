macro_rules! deps {
    () => {
        ContextSelectionSet!();
        OutputType!();
        Registry!();
        Field!();
        ServerResult!();
    };
}

macro_rules! impl_output_slice_for_smart_ptr {
    () => {
        deps!();
        macro_rules ! impl_output_slice_for_smart_ptr { ($ ty : ty) => { # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType > OutputType for $ ty { fn type_name () -> Cow <'static , str > { Cow :: Owned (format ! ("[{}]" , T :: qualified_type_name ())) } fn qualified_type_name () -> String { format ! ("[{}]!" , T :: qualified_type_name ()) } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) ; Self :: qualified_type_name () } async fn resolve (& self , ctx : & ContextSelectionSet <'_ >, field : & Positioned < Field >,) -> ServerResult < Value > { resolve_list (ctx , field , self . iter () , Some (self . len ())) . await } } } ; }
    };
}

impl_output_slice_for_smart_ptr!();