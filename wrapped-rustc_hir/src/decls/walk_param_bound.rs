macro_rules! deps {
    () => {
        Visitor!();
        GenericBound!();
    };
}

macro_rules! walk_param_bound {
    () => {
        deps!();
        pub fn walk_param_bound < 'v , V : Visitor < 'v > > (visitor : & mut V , bound : & 'v GenericBound < 'v > ,) -> V :: Result { match * bound { GenericBound :: Trait (ref typ) => visitor . visit_poly_trait_ref (typ) , GenericBound :: Outlives (ref lifetime) => visitor . visit_lifetime (lifetime) , GenericBound :: Use (args , _) => { walk_list ! (visitor , visit_precise_capturing_arg , args) ; V :: Result :: output () } } }
    };
}

walk_param_bound!();