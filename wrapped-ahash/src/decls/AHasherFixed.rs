macro_rules! deps {
    () => {
        AHasher!();
    };
}

macro_rules! AHasherFixed {
    () => {
        deps!();
        # [cfg (specialize)] pub (crate) struct AHasherFixed (pub AHasher) ;
    };
}

AHasherFixed!()