macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! validated_name {
    () => {
        deps!();
        fn validated_name (name : Cow < '_ , BStr >) -> Result < Cow < '_ , BStr > , Error > { name . iter () . all (| b | b . is_ascii_alphanumeric () || * b == b'-') . then_some (name) . ok_or (Error :: InvalidName) }
    };
}

validated_name!();