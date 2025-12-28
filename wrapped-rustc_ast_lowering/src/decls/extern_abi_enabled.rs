macro_rules! deps {
    () => {
        UnstableAbi!();
    };
}

macro_rules! extern_abi_enabled {
    () => {
        deps!();
        pub (crate) fn extern_abi_enabled (features : & rustc_feature :: Features , span : Span , abi : ExternAbi ,) -> Result < () , UnstableAbi > { extern_abi_stability (abi) . or_else (| unstable @ UnstableAbi { feature , .. } | { if features . enabled (feature) || span . allows_unstable (feature) { Ok (()) } else { Err (unstable) } }) }
    };
}

extern_abi_enabled!();