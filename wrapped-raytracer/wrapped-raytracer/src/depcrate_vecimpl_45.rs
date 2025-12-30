// Generated macro for impl_45 (impl)
macro_rules! Depcrate_vecimpl_45 {
() => {
// Module: crate::vec
// Provides: {"impl_45"}
// Dependencies: {}
impl Vec3 { pub fn x (& self) -> f32 { self . 0 } pub fn y (& self) -> f32 { self . 1 } pub fn z (& self) -> f32 { self . 2 } pub fn dot (& self , other : Vec3) -> f32 { self . 0 * other . 0 + self . 1 * other . 1 + self . 2 * other . 2 } pub fn cross (& self , other : Vec3) -> Vec3 { Vec3 (self . 1 * other . 2 - self . 2 * other . 1 , - (self . 0 * other . 2 - self . 2 * other . 0) , self . 0 * other . 1 - self . 1 * other . 0 ,) } pub fn squared_length (self) -> f32 { self . dot (self) } pub fn length (self) -> f32 { self . squared_length () . sqrt () } pub fn to_u8 (& self) -> [u8 ; 3] { fn u (f : f32) -> u8 { if f < 0.0 { 0 } else if f >= 1.0 { 255 } else { (f * 255.9) as i32 as u8 } } [u (self . 0) , u (self . 1) , u (self . 2)] } pub fn to_unit_vector (& self) -> Vec3 { * self / self . length () } }
};
}
