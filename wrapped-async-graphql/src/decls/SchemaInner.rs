macro_rules! deps {
    () => {
        SchemaEnv!();
        ValidationMode!();
        BoxResolverFn!();
        ExtensionFactory!();
    };
}

macro_rules! SchemaInner {
    () => {
        deps!();
        pub struct SchemaInner { pub (crate) env : SchemaEnv , pub (crate) types : IndexMap < String , Type > , extensions : Vec < Box < dyn ExtensionFactory > > , recursive_depth : usize , max_directives : Option < usize > , complexity : Option < usize > , depth : Option < usize > , validation_mode : ValidationMode , pub (crate) entity_resolver : Option < BoxResolverFn > , }
    };
}

SchemaInner!();