// Generated macro for range_check_with_overflow (function)
macro_rules! Depcrate_errorrange_check_with_overflow {
() => {
// Module: crate::error
// Provides: {"range_check_with_overflow"}
// Dependencies: {}
pub (crate) fn range_check_with_overflow < T : Ord + Into < i32 > + Copy > (value : T , field : & 'static str , bounds : core :: ops :: RangeInclusive < T > , overflow : Overflow ,) -> Result < T , RangeError > { if matches ! (overflow , Overflow :: Constrain) { Ok (value . clamp (* bounds . start () , * bounds . end ())) } else { range_check (value , field , bounds) } }
};
}
