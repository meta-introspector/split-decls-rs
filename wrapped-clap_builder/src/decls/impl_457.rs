macro_rules! deps {
    () => {
        ValuesRef!();
        AnyValue!();
    };
}

macro_rules! impl_457 {
    () => {
        deps!();
        # [doc = " Creates an empty iterator."] impl < 'a , T : 'a > Default for ValuesRef < 'a , T > { fn default () -> Self { static EMPTY : [Vec < AnyValue > ; 0] = [] ; ValuesRef { iter : EMPTY [..] . iter () . flatten () . map (| _ | unreachable ! ()) , len : 0 , } } }
    };
}

impl_457!();