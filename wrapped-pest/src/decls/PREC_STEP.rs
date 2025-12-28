macro_rules! deps {
    () => {
        Prec!();
    };
}

macro_rules! PREC_STEP {
    () => {
        deps!();
        const PREC_STEP : Prec = 10 ;
    };
}

PREC_STEP!()