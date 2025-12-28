macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_968 {
    () => {
        deps!();
        impl < Si , Item , U , Fut , F > With < Si , Item , U , Fut , F > where Si : Sink < Item > , F : FnMut (U) -> Fut , Fut : Future , { pub (super) fn new < E > (sink : Si , f : F) -> Self where Fut : Future < Output = Result < Item , E > > , E : From < Si :: Error > , { Self { state : None , sink , f , _phantom : PhantomData } } }
    };
}

impl_968!();