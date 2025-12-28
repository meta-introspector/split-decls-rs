macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Fail {
    () => {
        deps!();
        # [doc = " Parser implementation for [fail]"] pub struct Fail < O , E > { o : PhantomData < O > , e : PhantomData < E > , }
    };
}

Fail!();