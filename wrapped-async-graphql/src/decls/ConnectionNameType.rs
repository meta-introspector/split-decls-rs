macro_rules! deps {
    () => {
        OutputType!();
    };
}

macro_rules! ConnectionNameType {
    () => {
        deps!();
        # [doc = " Used to specify the connection name."] pub trait ConnectionNameType : Send + Sync { # [doc = " Returns the connection type name."] fn type_name < T : OutputType > () -> String ; }
    };
}

ConnectionNameType!()