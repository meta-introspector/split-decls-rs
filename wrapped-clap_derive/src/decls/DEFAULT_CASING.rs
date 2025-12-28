macro_rules! deps {
    () => {
        CasingStyle!();
    };
}

macro_rules! DEFAULT_CASING {
    () => {
        deps!();
        # [doc = " Default casing style for generated arguments."] pub (crate) const DEFAULT_CASING : CasingStyle = CasingStyle :: Kebab ;
    };
}

DEFAULT_CASING!();