macro_rules! Flag {
    () => {
        # [doc = "\nA defined flags value that may be named or unnamed.\n"] # [derive (Debug)] pub struct Flag < B > { name : & 'static str , value : B , }
    };
}

Flag!()