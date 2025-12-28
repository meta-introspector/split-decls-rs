macro_rules! deps {
    () => {
        Field!();
        ServerResult!();
        Registry!();
        OutputType!();
        ContextSelectionSet!();
    };
}

macro_rules! impl_863 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType + Hash + Eq > OutputType for HashSet < T > { fn type_name () -> Cow < 'static , str > { < StdHashSet < T > as OutputType > :: type_name () } fn qualified_type_name () -> String { < StdHashSet < T > as OutputType > :: qualified_type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { < StdHashSet < T > as OutputType > :: create_type_info (registry) } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { resolve_list (ctx , field , self , Some (self . len ())) . await } }
    };
}

impl_863!();