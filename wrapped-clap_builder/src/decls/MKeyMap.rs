macro_rules! deps {
    () => {
        Key!();
        Arg!();
    };
}

macro_rules! MKeyMap {
    () => {
        deps!();
        # [derive (Default , PartialEq , Eq , Debug , Clone)] pub (crate) struct MKeyMap { # [doc = " All of the arguments."] args : Vec < Arg > , # [doc = " Will be set after `_build()`."] keys : Vec < Key > , }
    };
}

MKeyMap!();