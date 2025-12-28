macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl From < Box < str > > for Box < PotentialUtf8 > { # [inline] fn from (other : Box < str >) -> Self { PotentialUtf8 :: from_boxed_str (other) } }
    };
}

impl_29!()