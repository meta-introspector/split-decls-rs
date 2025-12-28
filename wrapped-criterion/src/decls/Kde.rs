macro_rules! deps {
    () => {
        Float!();
        Kernel!();
        Sample!();
    };
}

macro_rules! Kde {
    () => {
        deps!();
        # [doc = " Univariate kernel density estimator"] pub struct Kde < 'a , A , K > where A : Float , K : Kernel < A > , { bandwidth : A , kernel : K , sample : & 'a Sample < A > , }
    };
}

Kde!()