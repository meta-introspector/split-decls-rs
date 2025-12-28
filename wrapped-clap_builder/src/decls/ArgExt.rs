macro_rules! deps {
    () => {
        Arg!();
        Extension!();
    };
}

macro_rules! ArgExt {
    () => {
        deps!();
        # [doc = " User-provided data that can be attached to an [`Arg`]"] # [cfg (feature = "unstable-ext")] pub trait ArgExt : Extension { }
    };
}

ArgExt!();