macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! String {
    () => {
        deps!();
        # [doc = " A key that represents a UTF-8 string."] pub type String = Any < validate :: String > ;
    };
}

String!();