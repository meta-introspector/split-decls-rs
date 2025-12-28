macro_rules! deps {
    () => {
        LazyFunctions!();
        Functions!();
        Result!();
        UnitRef!();
        LazyResult!();
        Error!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < R : gimli :: Reader > LazyFunctions < R > { pub (crate) fn new () -> Self { LazyFunctions (LazyResult :: new ()) } pub (crate) fn borrow (& self , unit : gimli :: UnitRef < R >) -> Result < & Functions < R > , Error > { self . 0 . get_or_init (| | Functions :: parse (unit)) . as_ref () . map_err (Error :: clone) } }
    };
}

impl_20!();