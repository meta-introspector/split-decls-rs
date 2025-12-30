// Generated macro for WINDOWS (const)
macro_rules! Depcrate_options_defaultsWINDOWS {
() => {
// Module: crate::options::defaults
// Provides: {"WINDOWS"}
// Dependencies: {}
# [doc = " Options to display sizes in the \"WINDOWS\" format."] # [doc = " Uses 1024 as the value of the `Kilo`, but displays decimal-style units (`kB`, not `KiB`)."] pub const WINDOWS : FormatSizeOptions = FormatSizeOptions { base_unit : BaseUnit :: Byte , kilo : Kilo :: Binary , units : Kilo :: Decimal , decimal_places : 2 , decimal_zeroes : 0 , fixed_at : None , long_units : false , space_after_value : true , suffix : "" , thousands_separator : None , } ;
};
}
