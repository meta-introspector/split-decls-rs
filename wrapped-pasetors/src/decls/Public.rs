macro_rules! Public {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone)] # [doc = " A public token."] pub struct Public ;
    };
}

Public!();