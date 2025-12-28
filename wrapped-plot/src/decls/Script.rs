macro_rules! Script {
    () => {
        # [doc = " Structs that can produce gnuplot code"] trait Script { # [doc = " Translates some configuration struct into gnuplot code"] fn script (& self) -> String ; }
    };
}

Script!();