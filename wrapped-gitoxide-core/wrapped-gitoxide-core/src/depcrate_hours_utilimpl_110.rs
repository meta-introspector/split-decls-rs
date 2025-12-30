// Generated macro for impl_110 (impl)
macro_rules! Depcrate_hours_utilimpl_110 {
() => {
// Module: crate::hours::util
// Provides: {"impl_110"}
// Dependencies: {}
impl FileStats { pub fn add (& mut self , other : & FileStats) -> & mut Self { self . added += other . added ; self . removed += other . removed ; self . modified += other . modified ; self } pub fn added (& self , other : & FileStats) -> Self { let mut a = * self ; a . add (other) ; a } pub fn sum (& self) -> f32 { (self . added + self . removed + self . modified) as f32 } }
};
}
