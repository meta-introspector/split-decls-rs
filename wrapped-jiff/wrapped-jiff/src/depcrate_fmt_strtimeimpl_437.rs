// Generated macro for impl_437 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_437 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_437"}
// Dependencies: {}
impl Custom for PosixCustom { fn format_datetime < W : Write > (& self , config : & Config < Self > , _ext : & Extension , tm : & BrokenDownTime , wtr : & mut W ,) -> Result < () , Error > { tm . format_with_config (config , "%a %b %e %H:%M:%S %Y" , wtr) } fn format_date < W : Write > (& self , config : & Config < Self > , _ext : & Extension , tm : & BrokenDownTime , wtr : & mut W ,) -> Result < () , Error > { tm . format_with_config (config , "%m/%d/%y" , wtr) } fn format_time < W : Write > (& self , config : & Config < Self > , _ext : & Extension , tm : & BrokenDownTime , wtr : & mut W ,) -> Result < () , Error > { tm . format_with_config (config , "%H:%M:%S" , wtr) } fn format_12hour_time < W : Write > (& self , config : & Config < Self > , _ext : & Extension , tm : & BrokenDownTime , wtr : & mut W ,) -> Result < () , Error > { tm . format_with_config (config , "%I:%M:%S %p" , wtr) } }
};
}
