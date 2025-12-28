macro_rules! deps {
    () => {
        Runtime!();
    };
}

macro_rules! RT {
    () => {
        deps!();
        static RT : Lazy < Mutex < Runtime > > = Lazy :: new (Default :: default) ;
    };
}

RT!();