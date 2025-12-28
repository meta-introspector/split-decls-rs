macro_rules! deps {
    () => {
        Declaration!();
    };
}

macro_rules! Uses {
    () => {
        deps!();
        pub trait Uses < T : Declaration > { fn uses (& self , other : & T) -> bool ; }
    };
}

Uses!()