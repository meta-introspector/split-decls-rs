macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_947 {
    () => {
        deps!();
        impl Default for Name { fn default () -> Name { Name :: Short ([0 ; 8]) } }
    };
}

impl_947!();