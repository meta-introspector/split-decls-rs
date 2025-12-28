macro_rules! deps {
    () => {
        EnvSnapshot!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Default for EnvSnapshot { fn default () -> EnvSnapshot { EnvSnapshot { vars : env :: vars_os () . collect () } } }
    };
}

impl_18!()