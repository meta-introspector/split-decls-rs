// Generated macro for range_check (function)
macro_rules! Depcrate_errorrange_check {
() => {
// Module: crate::error
// Provides: {"range_check"}
// Dependencies: {}
pub (crate) fn range_check < T : Ord + Into < i32 > + Copy > (value : T , field : & 'static str , bounds : core :: ops :: RangeInclusive < T > ,) -> Result < T , RangeError > { if ! bounds . contains (& value) { return Err (RangeError { field , value : value . into () , min : (* bounds . start ()) . into () , max : (* bounds . end ()) . into () , }) ; } Ok (value) }
};
}
