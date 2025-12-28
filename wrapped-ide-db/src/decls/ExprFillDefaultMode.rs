macro_rules! ExprFillDefaultMode {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq , Default)] pub enum ExprFillDefaultMode { # [default] Todo , Default , Underscore , }
    };
}

ExprFillDefaultMode!();