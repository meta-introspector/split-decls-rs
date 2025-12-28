macro_rules! deps {
    () => {
        ContainerAttributes!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Default for ContainerAttributes { fn default () -> Self { Self { crate_name : "::bincode" . to_string () , bounds : None , decode_bounds : None , decode_context : None , encode_bounds : None , borrow_decode_bounds : None , } } }
    };
}

impl_1!();