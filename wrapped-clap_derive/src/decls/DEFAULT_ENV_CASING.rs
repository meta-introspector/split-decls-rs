macro_rules! deps {
    () => {
        CasingStyle!();
    };
}

macro_rules! DEFAULT_ENV_CASING {
    () => {
        deps!();
        # [doc = " Default casing style for environment variables"] pub (crate) const DEFAULT_ENV_CASING : CasingStyle = CasingStyle :: ScreamingSnake ;
    };
}

DEFAULT_ENV_CASING!();