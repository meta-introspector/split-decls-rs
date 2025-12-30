// Generated macro for impl_378 (impl)
macro_rules! Depcrate_dimension_units_optionsimpl_378 {
() => {
// Module: crate::dimension::units::options
// Provides: {"impl_378"}
// Dependencies: {}
impl From < Width > for tinystr :: TinyStr8 { fn from (width : Width) -> Self { match width { Width :: Long => "long" . parse () . unwrap () , Width :: Short => "short" . parse () . unwrap () , Width :: Narrow => "narrow" . parse () . unwrap () , } } }
};
}
