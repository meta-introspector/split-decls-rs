macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_970 {
    () => {
        deps!();
        impl < S , Item , U , Fut , F > Stream for With < S , Item , U , Fut , F > where S : Stream + Sink < Item > , F : FnMut (U) -> Fut , Fut : Future , { type Item = S :: Item ; delegate_stream ! (sink) ; }
    };
}

impl_970!();