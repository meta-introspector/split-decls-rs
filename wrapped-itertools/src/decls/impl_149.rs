macro_rules! deps {
    () => {
        EitherOrBoth!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < T > EitherOrBoth < T , T > { # [doc = " Return either value of left, right, or apply a function `f` to both values if both are present."] # [doc = " The input function has to return the same type as both Right and Left carry."] # [doc = ""] # [doc = " This function can be used to preferably extract the left resp. right value,"] # [doc = " but fall back to the other (i.e. right resp. left) if the preferred one is not present."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use itertools::EitherOrBoth;"] # [doc = " assert_eq!(EitherOrBoth::Both(3, 7).reduce(u32::max), 7);"] # [doc = " assert_eq!(EitherOrBoth::Left(3).reduce(u32::max), 3);"] # [doc = " assert_eq!(EitherOrBoth::Right(7).reduce(u32::max), 7);"] # [doc = ""] # [doc = " // Extract the left value if present, fall back to the right otherwise."] # [doc = " assert_eq!(EitherOrBoth::Left(\"left\").reduce(|l, _r| l), \"left\");"] # [doc = " assert_eq!(EitherOrBoth::Right(\"right\").reduce(|l, _r| l), \"right\");"] # [doc = " assert_eq!(EitherOrBoth::Both(\"left\", \"right\").reduce(|l, _r| l), \"left\");"] # [doc = " ```"] pub fn reduce < F > (self , f : F) -> T where F : FnOnce (T , T) -> T , { match self { Left (a) => a , Right (b) => b , Both (a , b) => f (a , b) , } } }
    };
}

impl_149!()