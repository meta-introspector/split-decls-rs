macro_rules! deps {
    () => {
        IntrospectionMode!();
        MetaDirective!();
        MetaType!();
    };
}

macro_rules! Registry {
    () => {
        deps!();
        # [doc = " A type registry for build schemas"] # [derive (Default)] pub struct Registry { pub types : BTreeMap < String , MetaType > , pub directives : BTreeMap < String , MetaDirective > , pub implements : HashMap < String , IndexSet < String > > , pub query_type : String , pub mutation_type : Option < String > , pub subscription_type : Option < String > , pub introspection_mode : IntrospectionMode , pub enable_federation : bool , pub federation_subscription : bool , pub ignore_name_conflicts : HashSet < String > , pub enable_suggestions : bool , }
    };
}

Registry!()