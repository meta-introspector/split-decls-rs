macro_rules! deps {
    () => {
        RawValues!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        # [doc = " Creates an empty iterator."] impl Default for RawValues < '_ > { fn default () -> Self { static EMPTY : [Vec < OsString > ; 0] = [] ; RawValues { iter : EMPTY [..] . iter () . flatten () . map (| _ | unreachable ! ()) , len : 0 , } } }
    };
}

impl_462!();