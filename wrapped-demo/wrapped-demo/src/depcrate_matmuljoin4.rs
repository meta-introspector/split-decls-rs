// Generated macro for join4 (function)
macro_rules! Depcrate_matmuljoin4 {
() => {
// Module: crate::matmul
// Provides: {"join4"}
// Dependencies: {}
fn join4 < F1 , F2 , F3 , F4 , R1 , R2 , R3 , R4 > (f1 : F1 , f2 : F2 , f3 : F3 , f4 : F4) -> (R1 , R2 , R3 , R4) where F1 : FnOnce () -> R1 + Send , R1 : Send , F2 : FnOnce () -> R2 + Send , R2 : Send , F3 : FnOnce () -> R3 + Send , R3 : Send , F4 : FnOnce () -> R4 + Send , R4 : Send , { let ((r1 , r2) , (r3 , r4)) = rayon :: join (| | rayon :: join (f1 , f2) , | | rayon :: join (f3 , f4)) ; (r1 , r2 , r3 , r4) }
};
}
