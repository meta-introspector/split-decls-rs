// Generated macro for impl_31 (impl)
macro_rules! Depcrate_extimpl_31 {
() => {
// Module: crate::ext
// Provides: {"impl_31"}
// Dependencies: {}
impl FormatterExt for Formatter < '_ > { fn pad_with_width (& mut self , args_width : usize , args : Arguments < '_ >) -> Result { let Some (final_width) = self . width () else { return self . write_fmt (args) ; } ; let Some (fill_width @ 1 ..) = final_width . checked_sub (args_width) else { return self . write_fmt (args) ; } ; let alignment = self . align () . unwrap_or (Alignment :: Left) ; let fill = self . fill () ; let left_fill_width = match alignment { Alignment :: Left => 0 , Alignment :: Right => fill_width , Alignment :: Center => fill_width / 2 , } ; let right_fill_width = match alignment { Alignment :: Left => fill_width , Alignment :: Right => 0 , Alignment :: Center => (fill_width + 1) / 2 , } ; for _ in 0 .. left_fill_width { self . write_char (fill) ? ; } self . write_fmt (args) ? ; for _ in 0 .. right_fill_width { self . write_char (fill) ? ; } Ok (()) } }
};
}
