macro_rules! deps {
    () => {
        Options!();
        Never!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl Default for Options < fn () -> crate :: cache :: Never > { fn default () -> Self { Options { check : Default :: default () , traversal : Default :: default () , thread_limit : None , make_pack_lookup_cache : | | crate :: cache :: Never , } } }
    };
}

impl_261!();