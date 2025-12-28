macro_rules! deps {
    () => {
        Distribution!();
    };
}

macro_rules! Bernoulli {
    () => {
        deps!();
        # [doc = " The [Bernoulli distribution](https://en.wikipedia.org/wiki/Bernoulli_distribution) `Bernoulli(p)`."] # [doc = ""] # [doc = " This distribution describes a single boolean random variable, which is true"] # [doc = " with probability `p` and false with probability `1 - p`."] # [doc = " It is a special case of the Binomial distribution with `n = 1`."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the Bernoulli distribution with `p = 0.1`,"] # [doc = " `p = 0.5`, and `p = 0.9`."] # [doc = ""] # [doc = " ![Bernoulli distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/bernoulli.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use rand::distr::{Bernoulli, Distribution};"] # [doc = ""] # [doc = " let d = Bernoulli::new(0.3).unwrap();"] # [doc = " let v = d.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from a Bernoulli distribution\", v);"] # [doc = " ```"] # [doc = ""] # [doc = " # Precision"] # [doc = ""] # [doc = " This `Bernoulli` distribution uses 64 bits from the RNG (a `u64`),"] # [doc = " so only probabilities that are multiples of 2<sup>-64</sup> can be"] # [doc = " represented."] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Bernoulli { # [doc = " Probability of success, relative to the maximal integer."] p_int : u64 , }
    };
}

Bernoulli!()