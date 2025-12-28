macro_rules! deps {
    () => {
        Branch!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Default for Branch { fn default () -> Self { Branch :: Name ("HEAD" . into ()) } }
    };
}

impl_9!();