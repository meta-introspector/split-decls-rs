macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! Bounds {
    () => {
        deps!();
        # [doc = " Bounds on type parameters."] # [derive (Clone)] pub (crate) struct Bounds { pub bounds : Vec < (Symbol , Vec < Path >) > , }
    };
}

Bounds!()