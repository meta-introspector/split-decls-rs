macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_942 {
    () => {
        deps!();
        impl < Si , Item , E > Sink < Item > for SinkErrInto < Si , Item , E > where Si : Sink < Item > , Si :: Error : Into < E > , { type Error = E ; delegate_sink ! (sink , Item) ; }
    };
}

impl_942!();