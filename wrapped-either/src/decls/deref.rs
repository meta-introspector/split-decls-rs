macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! deref {
    () => {
        deps!();
        # [test] fn deref () { use std :: string :: String ; fn is_str (_ : & str) { } let value : Either < String , & str > = Left (String :: from ("test")) ; is_str (& value) ; }
    };
}

deref!();