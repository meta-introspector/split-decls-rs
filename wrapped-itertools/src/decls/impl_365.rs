macro_rules! deps {
    () => {
        MinMaxResult!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl < T : Clone > MinMaxResult < T > { # [doc = " `into_option` creates an `Option` of type `(T, T)`. The returned `Option`"] # [doc = " has variant `None` if and only if the `MinMaxResult` has variant"] # [doc = " `NoElements`. Otherwise `Some((x, y))` is returned where `x <= y`."] # [doc = " If the `MinMaxResult` has variant `OneElement(x)`, performing this"] # [doc = " operation will make one clone of `x`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::MinMaxResult::{self, MinMax, NoElements, OneElement};"] # [doc = ""] # [doc = " let r: MinMaxResult<i32> = NoElements;"] # [doc = " assert_eq!(r.into_option(), None);"] # [doc = ""] # [doc = " let r = OneElement(1);"] # [doc = " assert_eq!(r.into_option(), Some((1, 1)));"] # [doc = ""] # [doc = " let r = MinMax(1, 2);"] # [doc = " assert_eq!(r.into_option(), Some((1, 2)));"] # [doc = " ```"] pub fn into_option (self) -> Option < (T , T) > { match self { Self :: NoElements => None , Self :: OneElement (x) => Some ((x . clone () , x)) , Self :: MinMax (x , y) => Some ((x , y)) , } } }
    };
}

impl_365!();