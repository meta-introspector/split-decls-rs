macro_rules! deps {
    () => {
        Default!();
        Options!();
    };
}

macro_rules! impl_942 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { prefix_from_spec_as_filter_on_remote : true , handshake_parameters : Vec :: new () , extra_refspecs : Vec :: new () , } } }
    };
}

impl_942!();