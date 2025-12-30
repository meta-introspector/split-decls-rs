// Generated macro for impl_62 (impl)
macro_rules! Depcrate_core_geometryimpl_62 {
() => {
// Module: crate::core::geometry
// Provides: {"impl_62"}
// Dependencies: {}
impl Point { pub fn zero () -> Point { Self { x : 0. , y : 0. } } pub fn new (x : f64 , y : f64) -> Point { Self { x , y } } pub fn splat (s : f64) -> Point { Point :: new (s , s) } pub fn neg (& self) -> Point { Point :: new (- self . x , - self . y) } pub fn add (& self , other : Point) -> Point { Point :: new (self . x + other . x , self . y + other . y) } pub fn sub (& self , other : Point) -> Point { self . add (other . neg ()) } pub fn distance_to (& self , other : Point) -> f64 { let d = self . sub (other) ; (d . x * d . x + d . y * d . y) . sqrt () } pub fn length (& self) -> f64 { Point :: zero () . distance_to (* self) } pub fn scale (& self , s : f64) -> Point { Point :: new (self . x * s , self . y * s) } pub fn transpose (& self) -> Point { Point :: new (self . y , self . x) } pub fn rotate_around (& self , center : Point , angle : f64) -> Point { let normalized = self . sub (center) ; let rotated = normalized . rotate (angle) ; rotated . add (center) } pub fn rotate (& self , angle : f64) -> Point { let x = self . x ; let y = self . y ; Point :: new (x * angle . cos () - y * angle . sin () , x * angle . sin () + y * angle . cos () ,) } }
};
}
