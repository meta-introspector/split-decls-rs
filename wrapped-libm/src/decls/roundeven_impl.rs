macro_rules! deps {
    () => {
        Round!();
        Float!();
    };
}

macro_rules! roundeven_impl {
    () => {
        deps!();
        # [inline] pub fn roundeven_impl < F : Float > (x : F) -> F { super :: generic :: rint_round (x , Round :: Nearest) . val }
    };
}

roundeven_impl!();