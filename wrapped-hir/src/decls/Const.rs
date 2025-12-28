macro_rules! Const {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Const { pub (crate) id : ConstId , }
    };
}

Const!();