// Generated macro for impl_108 (impl)
macro_rules! Depcrate_primimpl_108 {
() => {
// Module: crate::prim
// Provides: {"impl_108"}
// Dependencies: {}
impl ModularCoreOps < u128 , & u128 > for u128 { type Output = u128 ; # [inline] fn addm (self , rhs : u128 , m : & u128) -> u128 { if let Some (ab) = self . checked_add (rhs) { ab % m } else { udouble :: widening_add (self , rhs) % * m } } # [inline] fn subm (self , rhs : u128 , m : & u128) -> u128 { if self >= rhs { (self - rhs) % m } else { ((rhs - self) % m) . negm (m) } } # [inline] fn mulm (self , rhs : u128 , m : & u128) -> u128 { if let Some (ab) = self . checked_mul (rhs) { ab % m } else { udouble :: widening_mul (self , rhs) % * m } } }
};
}
