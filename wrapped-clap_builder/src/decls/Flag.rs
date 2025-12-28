macro_rules! deps {
    () => {
        Arg!();
        Command!();
    };
}

macro_rules! Flag {
    () => {
        deps!();
        # [derive (Eq)] enum Flag < 'a > { Command (String , & 'a str) , Arg (String , & 'a str) , }
    };
}

Flag!();