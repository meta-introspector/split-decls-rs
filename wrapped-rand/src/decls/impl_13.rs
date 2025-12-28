macro_rules! deps {
    () => {
        Bernoulli!();
        BernoulliError!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Bernoulli { # [doc = " Construct a new `Bernoulli` with the given probability of success `p`."] # [doc = ""] # [doc = " # Precision"] # [doc = ""] # [doc = " For `p = 1.0`, the resulting distribution will always generate true."] # [doc = " For `p = 0.0`, the resulting distribution will always generate false."] # [doc = ""] # [doc = " This method is accurate for any input `p` in the range `[0, 1]` which is"] # [doc = " a multiple of 2<sup>-64</sup>. (Note that not all multiples of"] # [doc = " 2<sup>-64</sup> in `[0, 1]` can be represented as a `f64`.)"] # [inline] pub fn new (p : f64) -> Result < Bernoulli , BernoulliError > { if ! (0.0 .. 1.0) . contains (& p) { if p == 1.0 { return Ok (Bernoulli { p_int : ALWAYS_TRUE }) ; } return Err (BernoulliError :: InvalidProbability) ; } Ok (Bernoulli { p_int : (p * SCALE) as u64 , }) } # [doc = " Construct a new `Bernoulli` with the probability of success of"] # [doc = " `numerator`-in-`denominator`. I.e. `new_ratio(2, 3)` will return"] # [doc = " a `Bernoulli` with a 2-in-3 chance, or about 67%, of returning `true`."] # [doc = ""] # [doc = " return `true`. If `numerator == 0` it will always return `false`."] # [doc = " For `numerator > denominator` and `denominator == 0`, this returns an"] # [doc = " error. Otherwise, for `numerator == denominator`, samples are always"] # [doc = " true; for `numerator == 0` samples are always false."] # [inline] pub fn from_ratio (numerator : u32 , denominator : u32) -> Result < Bernoulli , BernoulliError > { if numerator > denominator || denominator == 0 { return Err (BernoulliError :: InvalidProbability) ; } if numerator == denominator { return Ok (Bernoulli { p_int : ALWAYS_TRUE }) ; } let p_int = ((f64 :: from (numerator) / f64 :: from (denominator)) * SCALE) as u64 ; Ok (Bernoulli { p_int }) } # [inline] # [doc = " Returns the probability (`p`) of the distribution."] # [doc = ""] # [doc = " This value may differ slightly from the input due to loss of precision."] pub fn p (& self) -> f64 { if self . p_int == ALWAYS_TRUE { 1.0 } else { (self . p_int as f64) / SCALE } } }
    };
}

impl_13!();