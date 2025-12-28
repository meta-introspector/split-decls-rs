macro_rules! deps {
    () => {
        Err!();
        Error!();
        Into!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < E > Err < E > { # [doc = " Tests if the result is Incomplete"] pub fn is_incomplete (& self) -> bool { matches ! (self , Err :: Incomplete (..)) } # [doc = " Applies the given function to the inner error"] pub fn map < E2 , F > (self , f : F) -> Err < E2 > where F : FnOnce (E) -> E2 , { match self { Err :: Incomplete (n) => Err :: Incomplete (n) , Err :: Failure (t) => Err :: Failure (f (t)) , Err :: Error (t) => Err :: Error (f (t)) , } } # [doc = " Automatically converts between errors if the underlying type supports it"] pub fn convert < F > (e : Err < F >) -> Self where E : From < F > , { e . map (crate :: lib :: std :: convert :: Into :: into) } }
    };
}

impl_148!()