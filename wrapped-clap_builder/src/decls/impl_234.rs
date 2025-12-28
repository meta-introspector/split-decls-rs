macro_rules! deps {
    () => {
        StyledStr!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl Default for & '_ StyledStr { fn default () -> Self { static DEFAULT : StyledStr = StyledStr :: new () ; & DEFAULT } }
    };
}

impl_234!();