macro_rules! deps {
    () => {
        Lines!();
        Error!();
        LazyResult!();
        Result!();
        LazyLines!();
        UnitRef!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl LazyLines { pub (crate) fn new () -> Self { LazyLines (LazyResult :: new ()) } pub (crate) fn borrow < R : gimli :: Reader > (& self , dw_unit : gimli :: UnitRef < R > , ilnp : & gimli :: IncompleteLineProgram < R , R :: Offset > ,) -> Result < & Lines , Error > { self . 0 . get_or_init (| | Lines :: parse (dw_unit , ilnp . clone ())) . as_ref () . map_err (Error :: clone) } }
    };
}

impl_36!();