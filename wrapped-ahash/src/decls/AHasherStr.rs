macro_rules! deps {
    () => {
        AHasher!();
    };
}

macro_rules! AHasherStr {
    () => {
        deps!();
        # [cfg (specialize)] pub (crate) struct AHasherStr (pub AHasher) ;
    };
}

AHasherStr!()