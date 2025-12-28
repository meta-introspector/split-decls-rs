macro_rules! EitherOrBoth {
    () => {
        # [doc = " Value that either holds a single A or B, or both."] # [derive (Clone , PartialEq , Eq , Hash , Debug)] pub enum EitherOrBoth < A , B = A > { # [doc = " Both values are present."] Both (A , B) , # [doc = " Only the left value of type `A` is present."] Left (A) , # [doc = " Only the right value of type `B` is present."] Right (B) , }
    };
}

EitherOrBoth!();