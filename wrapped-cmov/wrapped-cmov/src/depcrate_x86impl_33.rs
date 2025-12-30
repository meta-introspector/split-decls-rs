// Generated macro for impl_33 (impl)
macro_rules! Depcrate_x86impl_33 {
() => {
// Module: crate::x86
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] impl Cmov for u64 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { cmov ! ("cmovnz {1:r}, {2:r}" , self , value , condition) ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { cmov ! ("cmovz {1:r}, {2:r}" , self , value , condition) ; } }
};
}
