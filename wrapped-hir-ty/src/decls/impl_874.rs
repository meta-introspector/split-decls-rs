macro_rules! deps {
    () => {
        ProjectionStore!();
    };
}

macro_rules! impl_874 {
    () => {
        deps!();
        impl Default for ProjectionStore < '_ > { fn default () -> Self { let mut this = Self { id_to_proj : Default :: default () , proj_to_id : Default :: default () } ; this . intern (Box :: new ([])) ; this } }
    };
}

impl_874!()