macro_rules! deps {
    () => {
        CrlfRoundTripCheck!();
        RoundTripCheck!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl CrlfRoundTripCheck { pub (crate) fn to_eol_roundtrip_check (self , rela_path : & Path) -> Option < eol :: convert_to_git :: RoundTripCheck < '_ > > { match self { CrlfRoundTripCheck :: Fail => Some (eol :: convert_to_git :: RoundTripCheck :: Fail { rela_path }) , CrlfRoundTripCheck :: Warn => Some (eol :: convert_to_git :: RoundTripCheck :: Warn { rela_path }) , CrlfRoundTripCheck :: Skip => None , } } }
    };
}

impl_115!();