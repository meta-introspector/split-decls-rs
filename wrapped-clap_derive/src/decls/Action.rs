macro_rules! deps {
    () => {
        Method!();
    };
}

macro_rules! Action {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) enum Action { Explicit (Method) , Implicit (Ident) , }
    };
}

Action!();