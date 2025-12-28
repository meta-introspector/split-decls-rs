macro_rules! deps {
    () => {
        Formatter!();
    };
}

macro_rules! DefaultVisitSource {
    () => {
        deps!();
        struct DefaultVisitSource < 'a > (& 'a mut Formatter) ;
    };
}

DefaultVisitSource!()