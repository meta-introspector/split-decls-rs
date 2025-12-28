macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! FEDERATION_SCALARS {
    () => {
        deps!();
        const FEDERATION_SCALARS : & [& str] = & ["Any"] ;
    };
}

FEDERATION_SCALARS!();