macro_rules! deps {
    () => {
        RepoBuilder!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'cb > Default for RepoBuilder < 'cb > { fn default () -> Self { Self :: new () } }
    };
}

impl_94!()