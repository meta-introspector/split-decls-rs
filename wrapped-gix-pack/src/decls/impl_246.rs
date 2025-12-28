macro_rules! deps {
    () => {
        Never!();
        Options!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl Default for Options < fn () -> crate :: cache :: Never > { fn default () -> Self { Options { check : Default :: default () , thread_limit : None , make_pack_lookup_cache : | | crate :: cache :: Never , } } }
    };
}

impl_246!()