macro_rules! deps {
    () => {
        Sink!();
        SinkExt!();
    };
}

macro_rules! impl_941 {
    () => {
        deps!();
        impl < Si , E , Item > SinkErrInto < Si , Item , E > where Si : Sink < Item > , Si :: Error : Into < E > , { pub (super) fn new (sink : Si) -> Self { Self { sink : SinkExt :: sink_map_err (sink , Into :: into) } } delegate_access_inner ! (sink , Si , (.)) ; }
    };
}

impl_941!();