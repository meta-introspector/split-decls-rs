macro_rules! deps {
    () => {
        GlobError!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Error for GlobError { # [allow (deprecated)] fn description (& self) -> & str { self . error . description () } # [allow (unknown_lints , bare_trait_objects)] fn cause (& self) -> Option < & Error > { Some (& self . error) } }
    };
}

impl_8!()