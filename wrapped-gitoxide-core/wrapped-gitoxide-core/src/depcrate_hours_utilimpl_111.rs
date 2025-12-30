// Generated macro for impl_111 (impl)
macro_rules! Depcrate_hours_utilimpl_111 {
() => {
// Module: crate::hours::util
// Provides: {"impl_111"}
// Dependencies: {}
impl LineStats { pub fn add (& mut self , other : & LineStats) -> & mut Self { self . added += other . added ; self . removed += other . removed ; self } pub fn added (& self , other : & LineStats) -> Self { let mut a = * self ; a . add (other) ; a } pub fn sum (& self) -> f32 { (self . added + self . removed) as f32 } }
};
}
