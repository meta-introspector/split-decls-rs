// Generated macro for for_each_tuple_ (macro)
macro_rules! Depcrate_boundsfor_each_tuple_ {
() => {
// Module: crate::bounds
// Provides: {"for_each_tuple_"}
// Dependencies: {}
macro_rules ! for_each_tuple_ { ($ m : ident !!) => ($ m ! { }) ; ($ m : ident !! $ h : ident , $ ($ t : ident ,) *) => ($ m ! { $ h $ ($ t) * } for_each_tuple_ ! { $ m !! $ ($ t ,) * }) ; }
};
}
