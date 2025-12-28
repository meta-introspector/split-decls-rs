macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [cfg (any (test , feature = "std"))] # [doc = " `Either` implements `Error` if *both* `L` and `R` implement it."] # [doc = ""] # [doc = " Requires crate feature `\"std\"`"] impl < L , R > Error for Either < L , R > where L : Error , R : Error , { fn source (& self) -> Option < & (dyn Error + 'static) > { for_both ! (self , inner => inner . source ()) } # [allow (deprecated)] fn description (& self) -> & str { for_both ! (self , inner => inner . description ()) } # [allow (deprecated)] fn cause (& self) -> Option < & dyn Error > { for_both ! (self , inner => inner . cause ()) } }
    };
}

impl_58!();