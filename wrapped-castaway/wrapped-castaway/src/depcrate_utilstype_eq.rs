// Generated macro for type_eq (function)
macro_rules! Depcrate_utilstype_eq {
() => {
// Module: crate::utils
// Provides: {"type_eq"}
// Dependencies: {}
# [doc = " Determine if two static, generic types are equal to each other."] # [inline (always)] pub (crate) fn type_eq < T : 'static , U : 'static > () -> bool { mem :: size_of :: < T > () == mem :: size_of :: < U > () && mem :: align_of :: < T > () == mem :: align_of :: < U > () && mem :: needs_drop :: < T > () == mem :: needs_drop :: < U > () && TypeId :: of :: < T > () == TypeId :: of :: < U > () && type_name :: < T > () == type_name :: < U > () }
};
}
