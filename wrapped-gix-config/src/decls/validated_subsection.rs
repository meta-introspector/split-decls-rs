macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! validated_subsection {
    () => {
        deps!();
        fn validated_subsection (name : Cow < '_ , BStr >) -> Result < Cow < '_ , BStr > , Error > { is_valid_subsection (name . as_ref ()) . then_some (name) . ok_or (Error :: InvalidSubSection) }
    };
}

validated_subsection!();