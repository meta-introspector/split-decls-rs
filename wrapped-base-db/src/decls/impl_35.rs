macro_rules! deps {
    () => {
        ReleaseChannel!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl ReleaseChannel { pub fn as_str (self) -> & 'static str { match self { ReleaseChannel :: Stable => "stable" , ReleaseChannel :: Beta => "beta" , ReleaseChannel :: Nightly => "nightly" , } } # [allow (clippy :: should_implement_trait)] pub fn from_str (str : & str) -> Option < Self > { Some (match str { "" | "stable" => ReleaseChannel :: Stable , "nightly" => ReleaseChannel :: Nightly , _ if str . starts_with ("beta") => ReleaseChannel :: Beta , _ => return None , }) } }
    };
}

impl_35!();