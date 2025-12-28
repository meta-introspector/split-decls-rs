macro_rules! CanonicalCombiningClass {
    () => {
        # [doc = " This type exists as a shim for icu_properties CanonicalCombiningClass when the crate is disabled"] # [doc = " It should not be exposed to users."] # [cfg (not (feature = "icu_properties"))] # [derive (Copy , Clone , Eq , PartialEq , PartialOrd , Ord)] struct CanonicalCombiningClass (pub (crate) u8) ;
    };
}

CanonicalCombiningClass!();