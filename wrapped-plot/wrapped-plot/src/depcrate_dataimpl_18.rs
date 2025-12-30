// Generated macro for impl_18 (impl)
macro_rules! Depcrate_dataimpl_18 {
() => {
// Module: crate::data
// Provides: {"impl_18"}
// Dependencies: {}
impl Matrix { pub fn new < I > (rows : I , scale : < I :: Item as Row > :: Scale) -> Matrix where I : Iterator , I :: Item : Row , { let ncols = I :: Item :: ncols () ; let bytes_per_row = ncols * mem :: size_of :: < f64 > () ; let mut bytes = Vec :: with_capacity (rows . size_hint () . 0 * bytes_per_row) ; let mut nrows = 0 ; for row in rows { nrows += 1 ; row . append_to (& mut bytes , scale) ; } Matrix { bytes , ncols , nrows , } } pub fn bytes (& self) -> & [u8] { & self . bytes } pub fn ncols (& self) -> usize { self . ncols } pub fn nrows (& self) -> usize { self . nrows } }
};
}
