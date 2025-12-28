macro_rules! deps {
    () => {
        SubcommandCandidates!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl CommandExt for SubcommandCandidates { }
    };
}

impl_114!()