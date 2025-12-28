macro_rules! deps {
    () => {
        Properties!();
        Default!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Default for Properties { fn default () -> Properties { Properties { grids : map :: grid :: Map :: new () , hidden : false , label : None , logarithmic : false , range : None , scale_factor : 1. , tics : None , } } }
    };
}

impl_28!()