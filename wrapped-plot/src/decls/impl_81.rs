macro_rules! deps {
    () => {
        Properties!();
        Default!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl Default for Properties { fn default () -> Properties { Properties { axes : None , color : None , label : None , opacity : None , } } }
    };
}

impl_81!();