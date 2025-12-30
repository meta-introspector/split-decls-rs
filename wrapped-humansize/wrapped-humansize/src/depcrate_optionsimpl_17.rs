// Generated macro for impl_17 (impl)
macro_rules! Depcrate_optionsimpl_17 {
() => {
// Module: crate::options
// Provides: {"impl_17"}
// Dependencies: {}
impl FormatSizeOptions { pub const fn from (from : FormatSizeOptions) -> FormatSizeOptions { FormatSizeOptions { .. from } } pub const fn base_unit (mut self , base_unit : BaseUnit) -> FormatSizeOptions { self . base_unit = base_unit ; self } pub const fn kilo (mut self , kilo : Kilo) -> FormatSizeOptions { self . kilo = kilo ; self } pub const fn units (mut self , units : Kilo) -> FormatSizeOptions { self . units = units ; self } pub const fn decimal_places (mut self , decimal_places : usize) -> FormatSizeOptions { self . decimal_places = decimal_places ; self } pub const fn decimal_zeroes (mut self , decimal_zeroes : usize) -> FormatSizeOptions { self . decimal_zeroes = decimal_zeroes ; self } pub const fn fixed_at (mut self , fixed_at : Option < FixedAt >) -> FormatSizeOptions { self . fixed_at = fixed_at ; self } pub const fn long_units (mut self , long_units : bool) -> FormatSizeOptions { self . long_units = long_units ; self } pub const fn space_after_value (mut self , insert_space : bool) -> FormatSizeOptions { self . space_after_value = insert_space ; self } pub const fn suffix (mut self , suffix : & 'static str) -> FormatSizeOptions { self . suffix = suffix ; self } pub const fn thousands_separator (mut self , sep : Option < char >) -> FormatSizeOptions { self . thousands_separator = sep ; self } }
};
}
