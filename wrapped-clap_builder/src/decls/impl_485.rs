macro_rules! deps {
    () => {
        RawOccurrences!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl Default for RawOccurrences < '_ > { fn default () -> Self { static EMPTY : [Vec < OsString > ; 0] = [] ; RawOccurrences { iter : EMPTY [..] . iter () . map (| _ | unreachable ! ()) , } } }
    };
}

impl_485!()