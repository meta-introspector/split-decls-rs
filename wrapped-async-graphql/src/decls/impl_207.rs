macro_rules! deps {
    () => {
        KnownFragmentNames!();
        Visitor!();
        VisitorContext!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for KnownFragmentNames { fn enter_fragment_spread (& mut self , ctx : & mut VisitorContext < 'a > , fragment_spread : & 'a Positioned < FragmentSpread > ,) { if ! ctx . is_known_fragment (& fragment_spread . node . fragment_name . node) { ctx . report_error (vec ! [fragment_spread . pos] , format ! (r#"Unknown fragment: "{}""# , fragment_spread . node . fragment_name . node) ,) ; } } }
    };
}

impl_207!()