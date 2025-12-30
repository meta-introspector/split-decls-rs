// Generated macro for Montgomery (struct)
macro_rules! Depcrate_montyMontgomery {
() => {
// Module: crate::monty
// Provides: {"Montgomery"}
// Dependencies: {}
# [doc = " A modular reducer based on [Montgomery form](https://en.wikipedia.org/wiki/Montgomery_modular_multiplication#Montgomery_form), only supports odd modulus."] # [doc = ""] # [doc = " The generic type T represents the underlying integer representation for modular inverse `-m^-1 mod R`,"] # [doc = " and `R=2^B` will be used as the auxiliary modulus, where B is automatically selected"] # [doc = " based on the size of T."] # [derive (Debug , Clone , Copy)] pub struct Montgomery < T > { m : T , inv : T , }
};
}
