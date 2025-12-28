macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { slots : Default :: default () , object_hash : Default :: default () , use_multi_pack_index : true , current_dir : None , } } }
    };
}

impl_32!();