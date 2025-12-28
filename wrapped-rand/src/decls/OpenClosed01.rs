macro_rules! deps {
    () => {
        Uniform!();
        Open01!();
        StandardUniform!();
    };
}

macro_rules! OpenClosed01 {
    () => {
        deps!();
        # [doc = " A distribution to sample floating point numbers uniformly in the half-open"] # [doc = " interval `(0, 1]`, i.e. including 1 but not 0."] # [doc = ""] # [doc = " All values that can be generated are of the form `n * ε/2`. For `f32`"] # [doc = " the 24 most significant random bits of a `u32` are used and for `f64` the"] # [doc = " 53 most significant bits of a `u64` are used. The conversion uses the"] # [doc = " multiplicative method."] # [doc = ""] # [doc = " See also: [`StandardUniform`] which samples from `[0, 1)`, [`Open01`]"] # [doc = " which samples from `(0, 1)` and [`Uniform`] which samples from arbitrary"] # [doc = " ranges."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use rand::Rng;"] # [doc = " use rand::distr::OpenClosed01;"] # [doc = ""] # [doc = " let val: f32 = rand::rng().sample(OpenClosed01);"] # [doc = " println!(\"f32 from (0, 1): {}\", val);"] # [doc = " ```"] # [doc = ""] # [doc = " [`StandardUniform`]: crate::distr::StandardUniform"] # [doc = " [`Open01`]: crate::distr::Open01"] # [doc = " [`Uniform`]: crate::distr::uniform::Uniform"] # [derive (Clone , Copy , Debug , Default)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct OpenClosed01 ;
    };
}

OpenClosed01!()