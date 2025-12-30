// Generated macro for is_signedness_cast (function)
macro_rules! Depcrate_casts_utilsis_signedness_cast {
() => {
// Module: crate::casts::utils
// Provides: {"is_signedness_cast"}
// Dependencies: {}
# [doc = " Returns `Some` if the type cast is between 2 integral types that differ"] # [doc = " only in signedness, otherwise `None`. The value of `Some` is which"] # [doc = " signedness is casted to."] pub (super) fn is_signedness_cast (cast_from : Ty < '_ > , cast_to : Ty < '_ >) -> Option < CastTo > { match (cast_from . kind () , cast_to . kind ()) { (ty :: Int (from) , ty :: Uint (to)) if from . to_unsigned () == * to => Some (CastTo :: Unsigned) , (ty :: Uint (from) , ty :: Int (to)) if * from == to . to_unsigned () => Some (CastTo :: Signed) , _ => None , } }
};
}
