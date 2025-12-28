macro_rules! deps {
    () => {
        LabeledSample!();
        Error!();
        Float!();
        Sample!();
        Result!();
    };
}

macro_rules! classify {
    () => {
        deps!();
        # [doc = " Classifies the sample, and returns a labeled sample."] # [doc = ""] # [doc = " - Time: `O(N log N) where N = length`"] pub fn classify < A > (sample : & Sample < A >) -> LabeledSample < '_ , A > where A : Float , usize : cast :: From < A , Output = Result < usize , cast :: Error > > , { let (q1 , _ , q3) = sample . percentiles () . quartiles () ; let iqr = q3 - q1 ; let k_m = A :: cast (1.5_f32) ; let k_s = A :: cast (3) ; LabeledSample { fences : (q1 - k_s * iqr , q1 - k_m * iqr , q3 + k_m * iqr , q3 + k_s * iqr ,) , sample , } }
    };
}

classify!()