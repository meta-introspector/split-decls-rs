macro_rules! deps {
    () => {
        OutputType!();
    };
}

macro_rules! EdgeNameType {
    () => {
        deps!();
        # [doc = " Used to specify the edge name."] pub trait EdgeNameType : Send + Sync { # [doc = " Returns the edge type name."] fn type_name < T : OutputType > () -> String ; }
    };
}

EdgeNameType!();