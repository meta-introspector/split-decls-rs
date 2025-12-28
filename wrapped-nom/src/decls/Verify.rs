macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Verify {
    () => {
        deps!();
        # [doc = " Parser iplementation for verify"] pub struct Verify < F , G , O2 : ? Sized > { first : F , second : G , o2 : PhantomData < O2 > , }
    };
}

Verify!();