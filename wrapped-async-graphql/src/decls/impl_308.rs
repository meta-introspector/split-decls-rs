macro_rules! deps {
    () => {
        ServerError!();
        RuleError!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl From < RuleError > for ServerError { fn from (e : RuleError) -> Self { Self { message : e . message , source : None , locations : e . locations , path : Vec :: new () , extensions : None , } } }
    };
}

impl_308!()