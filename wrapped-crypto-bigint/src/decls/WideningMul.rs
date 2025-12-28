macro_rules! deps {
    () => {
        Integer!();
        ConcatenatingMul!();
    };
}

macro_rules! WideningMul {
    () => {
        deps!();
        # [doc = " Widening multiply: returns a value with a number of limbs equal to the sum of the inputs."] # [deprecated (since = "0.7.0" , note = "please use `ConcatenatingMul` instead")] pub trait WideningMul < Rhs = Self > : Sized { # [doc = " Output of the widening multiplication."] type Output : Integer ; # [doc = " Perform widening multiplication."] fn widening_mul (& self , rhs : Rhs) -> Self :: Output ; }
    };
}

WideningMul!();