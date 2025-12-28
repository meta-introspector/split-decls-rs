macro_rules! deps {
    () => {
        Distribution!();
    };
}

macro_rules! ChangeDistributions {
    () => {
        deps!();
        pub struct ChangeDistributions { pub mean : Distribution < f64 > , pub median : Distribution < f64 > , }
    };
}

ChangeDistributions!();