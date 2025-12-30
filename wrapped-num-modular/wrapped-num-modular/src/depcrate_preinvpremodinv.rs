// Generated macro for PreModInv (struct)
macro_rules! Depcrate_preinvPreModInv {
() => {
// Module: crate::preinv
// Provides: {"PreModInv"}
// Dependencies: {}
# [doc = " Pre-computing the modular inverse for fast divisibility check."] # [doc = ""] # [doc = " This struct stores the modular inverse of a divisor, and a limit for divisibility check."] # [doc = " See <https://math.stackexchange.com/a/1251328> for the explanation of the trick"] # [derive (Debug , Clone , Copy)] pub struct PreModInv < T > { d_inv : T , q_lim : T , }
};
}
