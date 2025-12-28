macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_978 {
    () => {
        deps!();
        impl < S , Item , U , St , F > Stream for WithFlatMap < S , Item , U , St , F > where S : Stream + Sink < Item > , F : FnMut (U) -> St , St : Stream < Item = Result < Item , S :: Error > > , { type Item = S :: Item ; delegate_stream ! (sink) ; }
    };
}

impl_978!()