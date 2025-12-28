macro_rules! NonZeroChar {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct NonZeroChar (char) ;
    };
}

NonZeroChar!();