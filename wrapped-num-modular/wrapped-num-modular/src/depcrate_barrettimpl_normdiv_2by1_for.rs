// Generated macro for impl_normdiv_2by1_for (macro)
macro_rules! Depcrate_barrettimpl_normdiv_2by1_for {
() => {
// Module: crate::barrett
// Provides: {"impl_normdiv_2by1_for"}
// Dependencies: {}
macro_rules ! impl_normdiv_2by1_for { ($ T : ty , $ D : ty) => { impl Normalized2by1Divisor <$ T > { # [doc = " Calculate the inverse m > 0 of a normalized divisor (fit in a word), such that"] # [doc = ""] # [doc = " (m + B) * divisor = B^2 - k for some 1 <= k <= divisor"] # [doc = ""] # [inline] pub const fn invert_word (divisor : $ T) -> $ T { let (m , _hi) = split (<$ D >:: MAX / extend (divisor)) ; debug_assert ! (_hi == 1) ; m } # [doc = " Initialize from a given normalized divisor."] # [doc = ""] # [doc = " The divisor must have top bit of 1"] # [inline] pub const fn new (divisor : $ T) -> Self { assert ! (divisor . leading_zeros () == 0) ; Self { divisor , m : Self :: invert_word (divisor) , } } # [doc = " Returns (a / divisor, a % divisor)"] # [inline] pub const fn div_rem_1by1 (& self , a : $ T) -> ($ T , $ T) { if a < self . divisor { (0 , a) } else { (1 , a - self . divisor) } } # [doc = " Returns (a / divisor, a % divisor)"] # [doc = " The result must fit in a single word."] # [inline] pub const fn div_rem_2by1 (& self , a : $ D) -> ($ T , $ T) { let (a_lo , a_hi) = split (a) ; debug_assert ! (a_hi < self . divisor) ; let (q0 , q1) = split (wmul (self . m , a_hi) + a) ; let q = q1 . wrapping_add (1) ; let r = a_lo . wrapping_sub (q . wrapping_mul (self . divisor)) ; let (_ , decrease) = split (extend (q0) . wrapping_sub (extend (r))) ; let mut q = q . wrapping_add (decrease) ; let mut r = r . wrapping_add (decrease & self . divisor) ; if r >= self . divisor { q += 1 ; r -= self . divisor ; } (q , r) } } } ; }
};
}
