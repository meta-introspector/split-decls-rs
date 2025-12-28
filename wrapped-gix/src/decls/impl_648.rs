macro_rules! deps {
    () => {
        NegotiationAlgorithm!();
        Fetch!();
        Tree!();
        RecurseSubmodules!();
    };
}

macro_rules! impl_648 {
    () => {
        deps!();
        impl Fetch { # [doc = " The `fetch.negotiationAlgorithm` key."] pub const NEGOTIATION_ALGORITHM : NegotiationAlgorithm = NegotiationAlgorithm :: new_with_validate ("negotiationAlgorithm" , & config :: Tree :: FETCH , validate :: NegotiationAlgorithm ,) ; # [doc = " The `fetch.recurseSubmodules` key."] # [cfg (feature = "attributes")] pub const RECURSE_SUBMODULES : RecurseSubmodules = RecurseSubmodules :: new_with_validate ("recurseSubmodules" , & config :: Tree :: FETCH , validate :: RecurseSubmodules) ; }
    };
}

impl_648!();