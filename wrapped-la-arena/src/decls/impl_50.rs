macro_rules! deps {
    () => {
        Arena!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T > Default for Arena < T > { fn default () -> Arena < T > { Arena { data : Vec :: new () } } }
    };
}

impl_50!()