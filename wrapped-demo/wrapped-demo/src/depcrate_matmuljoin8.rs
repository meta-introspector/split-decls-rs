// Generated macro for join8 (function)
macro_rules! Depcrate_matmuljoin8 {
() => {
// Module: crate::matmul
// Provides: {"join8"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] fn join8 < F1 , F2 , F3 , F4 , F5 , F6 , F7 , F8 , R1 , R2 , R3 , R4 , R5 , R6 , R7 , R8 > (f1 : F1 , f2 : F2 , f3 : F3 , f4 : F4 , f5 : F5 , f6 : F6 , f7 : F7 , f8 : F8 ,) -> (R1 , R2 , R3 , R4 , R5 , R6 , R7 , R8) where F1 : FnOnce () -> R1 + Send , R1 : Send , F2 : FnOnce () -> R2 + Send , R2 : Send , F3 : FnOnce () -> R3 + Send , R3 : Send , F4 : FnOnce () -> R4 + Send , R4 : Send , F5 : FnOnce () -> R5 + Send , R5 : Send , F6 : FnOnce () -> R6 + Send , R6 : Send , F7 : FnOnce () -> R7 + Send , R7 : Send , F8 : FnOnce () -> R8 + Send , R8 : Send , { let (((r1 , r2) , (r3 , r4)) , ((r5 , r6) , (r7 , r8))) = rayon :: join (| | rayon :: join (| | rayon :: join (f1 , f2) , | | rayon :: join (f3 , f4)) , | | rayon :: join (| | rayon :: join (f5 , f6) , | | rayon :: join (f7 , f8)) ,) ; (r1 , r2 , r3 , r4 , r5 , r6 , r7 , r8) }
};
}
