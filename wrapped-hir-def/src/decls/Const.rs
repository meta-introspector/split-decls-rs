macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! Const {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Const { # [doc = " `None` for `const _: () = ();`"] pub name : Option < Name > , pub (crate) visibility : RawVisibilityId , }
    };
}

Const!();