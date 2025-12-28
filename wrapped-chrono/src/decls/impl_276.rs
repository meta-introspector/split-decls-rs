macro_rules! deps {
    () => {
        Item!();
        Numeric!();
        Error!();
        Fixed!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl Item < '_ > { # [doc = " Convert items that contain a reference to the format string into an owned variant."] # [cfg (any (feature = "alloc" , feature = "std"))] pub fn to_owned (self) -> Item < 'static > { match self { Item :: Literal (s) => Item :: OwnedLiteral (Box :: from (s)) , Item :: Space (s) => Item :: OwnedSpace (Box :: from (s)) , Item :: Numeric (n , p) => Item :: Numeric (n , p) , Item :: Fixed (f) => Item :: Fixed (f) , Item :: OwnedLiteral (l) => Item :: OwnedLiteral (l) , Item :: OwnedSpace (s) => Item :: OwnedSpace (s) , Item :: Error => Item :: Error , } } }
    };
}

impl_276!();