macro_rules! Category {
    () => {
        # [doc = " Category of internally-represented number."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum Category { Infinity , NaN , Normal , Zero , }
    };
}

Category!()