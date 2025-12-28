macro_rules! deps {
    () => {
        X87DoubleExtended!();
    };
}

macro_rules! X87DoubleExtendedS {
    () => {
        deps!();
        # [doc = " Floating point semantics for [`X87DoubleExtended`]."] # [doc = ""] # [doc = " See that type for more details."] pub struct X87DoubleExtendedS ;
    };
}

X87DoubleExtendedS!()