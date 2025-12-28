macro_rules! deps {
    () => {
        ReferenceStyle!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl ReferenceStyle { # [track_caller] fn parse (arg : & str) -> Self { match arg { "full" => Self :: Full , "flat" => Self :: Flat , "skip-root" => Self :: SkipRoot , _ => invalid_reference () , } } }
    };
}

impl_59!();