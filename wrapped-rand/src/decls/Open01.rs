macro_rules! deps {
    () => {
        Uniform!();
        OpenClosed01!();
        StandardUniform!();
    };
}

macro_rules! Open01 {
    () => {
        deps!();
        # [doc = " A distribution to sample floating point numbers uniformly in the open"] # [doc = " interval `(0, 1)`, i.e. not including either endpoint."] # [doc = ""] # [doc = " All values that can be generated are of the form `n * ε + ε/2`. For `f32`"] # [doc = " the 23 most significant random bits of an `u32` are used, for `f64` 52 from"] # [doc = " an `u64`. The conversion uses a transmute-based method."] # [doc = ""] # [doc = " See also: [`StandardUniform`] which samples from `[0, 1)`, [`OpenClosed01`]"] # [doc = " which samples from `(0, 1]` and [`Uniform`] which samples from arbitrary"] # [doc = " ranges."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use rand::Rng;"] # [doc = " use rand::distr::Open01;"] # [doc = ""] # [doc = " let val: f32 = rand::rng().sample(Open01);"] # [doc = " println!(\"f32 from (0, 1): {}\", val);"] # [doc = " ```"] # [doc = ""] # [doc = " [`StandardUniform`]: crate::distr::StandardUniform"] # [doc = " [`OpenClosed01`]: crate::distr::OpenClosed01"] # [doc = " [`Uniform`]: crate::distr::uniform::Uniform"] # [derive (Clone , Copy , Debug , Default)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Open01 ;
    };
}

Open01!();