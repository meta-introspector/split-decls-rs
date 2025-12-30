// Generated macro for impl_30 (impl)
macro_rules! Depcrate_loggingimpl_30 {
() => {
// Module: crate::logging
// Provides: {"impl_30"}
// Dependencies: {}
impl log :: Log for KernelLogger { fn enabled (& self , _ : & Metadata < '_ >) -> bool { true } fn flush (& self) { } fn log (& self , record : & Record < '_ >) { if ! self . enabled (record . metadata ()) { return ; } let time ; let format_time = if self . time () { time = Microseconds (crate :: processor :: get_timer_ticks ()) ; format_args ! ("[{time}]") } else { format_args ! ("[            ]") } ; let core_id = crate :: arch :: core_local :: core_id () ; let level = ColorLevel (record . level ()) ; let target = record . target () ; let (crate_ , modules) = target . split_once ("::") . unwrap_or ((target , "")) ; let (_modules , module) = modules . rsplit_once ("::") . unwrap_or (("" , modules)) ; let target = if ! module . is_empty () && crate_ == "hermit" { module } else { crate_ } ; let format_target = if cfg ! (feature = "log-target") { format_args ! (" {target:<10}") } else { format_args ! ("") } ; let args = record . args () ; println ! ("{format_time}[{core_id}][{level}{format_target}] {args}") ; } }
};
}
