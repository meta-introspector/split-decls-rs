// Generated macro for impl_1222 (impl)
macro_rules! Depcrate_units_ratioimpl_1222 {
() => {
// Module: crate::units::ratio
// Provides: {"impl_1222"}
// Dependencies: {}
impl MulAssign < & SiPrefix > for IcuRatio { fn mul_assign (& mut self , rhs : & SiPrefix) { match rhs . base { Base :: Decimal => { * self *= IcuRatio :: ten () . pow (rhs . power as i32) ; } Base :: Binary => { * self *= IcuRatio :: two () . pow (rhs . power as i32) ; } } } }
};
}
