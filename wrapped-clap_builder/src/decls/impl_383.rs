macro_rules! deps {
    () => {
        ContextValue!();
        Result!();
        StyledStr!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl std :: fmt :: Display for ContextValue { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: None => "" . fmt (f) , Self :: Bool (v) => v . fmt (f) , Self :: String (v) => v . fmt (f) , Self :: Strings (v) => v . join (", ") . fmt (f) , Self :: StyledStr (v) => v . fmt (f) , Self :: StyledStrs (v) => { for (i , v) in v . iter () . enumerate () { if i != 0 { ", " . fmt (f) ? ; } v . fmt (f) ? ; } Ok (()) } Self :: Number (v) => v . fmt (f) , } } }
    };
}

impl_383!();