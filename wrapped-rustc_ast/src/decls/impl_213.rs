macro_rules! deps {
    () => {
        Const!();
        Safety!();
        Extern!();
        FnHeader!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl Default for FnHeader { fn default () -> FnHeader { FnHeader { safety : Safety :: Default , coroutine_kind : None , constness : Const :: No , ext : Extern :: None , } } }
    };
}

impl_213!();