// Generated macro for use_hint (function)
macro_rules! Depcrate_hintuse_hint {
() => {
// Module: crate::hint
// Provides: {"use_hint"}
// Dependencies: {}
# [allow (clippy :: integer_division_remainder_used)] fn use_hint < TwoGamma2 : Unsigned > (h : bool , r : Elem) -> Elem { let m : u32 = (BaseField :: Q - 1) / TwoGamma2 :: U32 ; let (r1 , r0) = r . decompose :: < TwoGamma2 > () ; let gamma2 = TwoGamma2 :: U32 / 2 ; if h && r0 . 0 <= gamma2 { Elem :: new ((r1 . 0 + 1) % m) } else if h && r0 . 0 >= BaseField :: Q - gamma2 { Elem :: new ((r1 . 0 + m - 1) % m) } else if h { unreachable ! () ; } else { r1 } }
};
}
