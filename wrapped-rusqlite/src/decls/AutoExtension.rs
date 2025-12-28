macro_rules! deps {
    () => {
        Connection!();
        Result!();
    };
}

macro_rules! AutoExtension {
    () => {
        deps!();
        # [doc = " Automatic extension initialization routine"] pub type AutoExtension = fn (Connection) -> Result < () > ;
    };
}

AutoExtension!();