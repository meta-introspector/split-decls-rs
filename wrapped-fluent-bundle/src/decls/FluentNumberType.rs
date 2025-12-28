macro_rules! FluentNumberType {
    () => {
        # [derive (Clone , Copy , Debug , Default , Eq , Hash , PartialEq)] pub enum FluentNumberType { # [default] Cardinal , Ordinal , }
    };
}

FluentNumberType!();