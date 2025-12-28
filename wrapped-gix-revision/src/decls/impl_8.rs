macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Default for Options < '_ > { fn default () -> Self { Options { max_candidates : 10 , name_by_oid : Default :: default () , fallback_to_oid : false , first_parent : false , } } }
    };
}

impl_8!();