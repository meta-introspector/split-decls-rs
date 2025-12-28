macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_943 {
    () => {
        deps!();
        impl < S , Item , E > Stream for SinkErrInto < S , Item , E > where S : Sink < Item > + Stream , S :: Error : Into < E > , { type Item = S :: Item ; delegate_stream ! (sink) ; }
    };
}

impl_943!()