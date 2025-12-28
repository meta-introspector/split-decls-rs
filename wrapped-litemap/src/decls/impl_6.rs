macro_rules! impl_6 {
    () => {
        # [cfg (feature = "alloc")] impl < K , V > LiteMap < K , V > { # [doc = " Construct a new [`LiteMap`] backed by Vec  "] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*  "] pub const fn new_vec () -> Self { Self { values : alloc :: vec :: Vec :: new () , _key_type : PhantomData , _value_type : PhantomData , } } }
    };
}

impl_6!()