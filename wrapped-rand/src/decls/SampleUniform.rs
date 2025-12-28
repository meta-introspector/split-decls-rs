macro_rules! deps {
    () => {
        Uniform!();
        UniformSampler!();
    };
}

macro_rules! SampleUniform {
    () => {
        deps!();
        # [doc = " Helper trait for creating objects using the correct implementation of"] # [doc = " [`UniformSampler`] for the sampling type."] # [doc = ""] # [doc = " See the [module documentation] on how to implement [`Uniform`] range"] # [doc = " sampling for a custom type."] # [doc = ""] # [doc = " [module documentation]: crate::distr::uniform"] pub trait SampleUniform : Sized { # [doc = " The `UniformSampler` implementation supporting type `X`."] type Sampler : UniformSampler < X = Self > ; }
    };
}

SampleUniform!()