macro_rules! deps {
    () => {
        Command!();
        Arg!();
    };
}

macro_rules! Flag {
    () => {
        deps!();
        # [derive (Eq)] enum Flag < 'a > { Command (String , & 'a str) , Arg (String , & 'a str) , }
    };
}

Flag!()