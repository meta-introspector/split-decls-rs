macro_rules! deps {
    () => {
        Truncate!();
    };
}

macro_rules! TruncateTarget {
    () => {
        deps!();
        # [doc = " A type that can be used with turbofish syntax in [`Truncate::truncate`]."] # [doc = ""] # [doc = " It is unlikely that you will want to use this trait directly. You are probably looking for the"] # [doc = " [`Truncate`] trait."] pub trait TruncateTarget < T > : sealed :: TruncateTargetSealed < T > { }
    };
}

TruncateTarget!()