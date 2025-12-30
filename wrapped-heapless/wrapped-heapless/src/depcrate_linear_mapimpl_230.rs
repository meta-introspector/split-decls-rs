// Generated macro for impl_230 (impl)
macro_rules! Depcrate_linear_mapimpl_230 {
() => {
// Module: crate::linear_map
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'a , K , V > VacantEntry < 'a , K , V > where K : Eq , { # [doc = " Get the key associated with this entry"] pub fn key (& self) -> & K { & self . key } # [doc = " Consumes this entry to yield to key associated with it"] pub fn into_key (self) -> K { self . key } # [doc = " Inserts this entry into to underlying map, yields a mutable reference to the inserted value."] # [doc = " If the map is at capacity the value is returned instead."] pub fn insert (self , value : V) -> Result < & 'a mut V , V > { self . map . buffer . push ((self . key , value)) . map_err (| (_k , v) | v) ? ; let idx = self . map . buffer . len () - 1 ; let r = & mut self . map . buffer [idx] ; Ok (& mut r . 1) } }
};
}
