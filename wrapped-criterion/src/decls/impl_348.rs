macro_rules! deps {
    () => {
        Float!();
        Estimates!();
        Kde!();
        Sample!();
        Kernel!();
        Bandwidth!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl < 'a , A , K > Kde < 'a , A , K > where A : 'a + Float , K : Kernel < A > , { # [doc = " Creates a new kernel density estimator from the `sample`, using a kernel and estimating"] # [doc = " the bandwidth using the method `bw`"] pub fn new (sample : & 'a Sample < A > , kernel : K , bw : Bandwidth) -> Kde < 'a , A , K > { Kde { bandwidth : bw . estimate (sample) , kernel , sample , } } # [doc = " Returns the bandwidth used by the estimator"] pub fn bandwidth (& self) -> A { self . bandwidth } # [doc = " Maps the KDE over `xs`"] # [doc = ""] # [doc = " - Multihreaded"] pub fn map (& self , xs : & [A]) -> Box < [A] > { # [cfg (feature = "rayon")] let iter = xs . par_iter () ; # [cfg (not (feature = "rayon"))] let iter = xs . iter () ; iter . map (| & x | self . estimate (x)) . collect :: < Vec < _ > > () . into_boxed_slice () } # [doc = " Estimates the probability density of `x`"] pub fn estimate (& self , x : A) -> A { let _0 = A :: cast (0) ; let slice = self . sample ; let h = self . bandwidth ; let n = A :: cast (slice . len ()) ; let sum = slice . iter () . fold (_0 , | acc , & x_i | acc + self . kernel . evaluate ((x - x_i) / h)) ; sum / (h * n) } }
    };
}

impl_348!()