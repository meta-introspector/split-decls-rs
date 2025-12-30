// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl fmt :: Display for Edition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Edition :: Edition2015 => "2015" , Edition :: Edition2018 => "2018" , Edition :: Edition2021 => "2021" , Edition :: Edition2024 => "2024" , }) } }
};
}
