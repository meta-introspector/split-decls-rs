macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Escaped {
    () => {
        deps!();
        # [doc = " Parser implementation for [escaped]"] pub struct Escaped < F , G , E > { normal : F , escapable : G , control_char : char , e : PhantomData < E > , }
    };
}

Escaped!()