macro_rules! deps {
    () => {
        Method!();
    };
}

macro_rules! ValueParser {
    () => {
        deps!();
        # [derive (Clone)] enum ValueParser { Explicit (Method) , Implicit (Ident) , }
    };
}

ValueParser!()