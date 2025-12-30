// Generated macro for parts (module)
macro_rules! Depcrate_duration_formatparts {
() => {
// Module: crate::duration::format
// Provides: {"parts"}
// Dependencies: {}
pub mod parts { # ! [allow (dead_code)] use crate :: duration :: validated_options :: Unit ; use writeable :: Part ; create_unit_parts ! (YEAR , Unit :: Year , MONTH , Unit :: Month , WEEK , Unit :: Week , DAY , Unit :: Day , HOUR , Unit :: Hour , MINUTE , Unit :: Minute , SECOND , Unit :: Second , MILLISECOND , Unit :: Millisecond , MICROSECOND , Unit :: Microsecond , NANOSECOND , Unit :: Nanosecond) ; pub const LITERAL : Part = Part { category : "duration" , value : "literal" , } ; }
};
}
