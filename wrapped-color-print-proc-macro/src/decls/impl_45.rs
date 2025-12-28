macro_rules! deps {
    () => {
        Change!();
        Action!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T > Action < T > where T : PartialEq , { # [doc = " Creates a new [`Action`]."] pub fn from_diff (old : Option < T > , new : Option < T >) -> Self { let eq = old == new ; match (old , new , eq) { (Some (old_val) , Some (_) , true) | (Some (old_val) , None , _) => Action :: Keep (old_val) , (_ , Some (new_val) , _) => Action :: Change (new_val) , _ => Action :: None , } } }
    };
}

impl_45!();