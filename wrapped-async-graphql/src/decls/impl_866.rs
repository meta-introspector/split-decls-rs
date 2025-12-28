macro_rules! deps {
    () => {
        ContextSelectionSet!();
        Field!();
        Registry!();
        OutputType!();
        ServerResult!();
    };
}

macro_rules! impl_866 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType > OutputType for LinkedList < T > { fn type_name () -> Cow < 'static , str > { Cow :: Owned (format ! ("[{}]" , T :: qualified_type_name ())) } fn qualified_type_name () -> String { format ! ("[{}]!" , T :: qualified_type_name ()) } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) ; Self :: qualified_type_name () } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { resolve_list (ctx , field , self , Some (self . len ())) . await } }
    };
}

impl_866!();