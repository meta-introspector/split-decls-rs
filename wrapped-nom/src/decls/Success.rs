macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Success {
    () => {
        deps!();
        # [doc = " Parser implementation for [success]"] pub struct Success < O : Clone , E > { val : O , e : PhantomData < E > , }
    };
}

Success!();