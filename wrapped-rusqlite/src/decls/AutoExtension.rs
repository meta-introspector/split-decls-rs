macro_rules! deps {
    () => {
        Result!();
        Connection!();
    };
}

macro_rules! AutoExtension {
    () => {
        deps!();
        # [doc = " Automatic extension initialization routine"] pub type AutoExtension = fn (Connection) -> Result < () > ;
    };
}

AutoExtension!()