macro_rules! Float {
    () => {
        # [doc = " This is an extension of `num_traits::float::Float` that adds safe"] # [doc = " casting and Sync + Send. Once `num_traits` has these features this"] # [doc = " can be removed."] pub trait Float : float :: Float + From < usize , Output = Self > + From < f32 , Output = Self > + Sync + Send { }
    };
}

Float!();