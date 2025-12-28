macro_rules! deps {
    () => {
        Properties!();
        Default!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl Default for Properties { fn default () -> Properties { Properties { boxed : false , hidden : false , justification : None , order : None , position : None , stacked : None , title : None , } } }
    };
}

impl_96!();