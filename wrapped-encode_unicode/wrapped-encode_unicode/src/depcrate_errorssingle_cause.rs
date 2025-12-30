// Generated macro for single_cause (macro)
macro_rules! Depcrate_errorssingle_cause {
() => {
// Module: crate::errors
// Provides: {"single_cause"}
// Dependencies: {}
macro_rules ! single_cause { ($ (# [$ doc : meta]) * $ err : ident => $ desc : expr) => { $ (# [$ doc]) * # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct $ err ; description ! { $ err , | _ | $ desc } } }
};
}
