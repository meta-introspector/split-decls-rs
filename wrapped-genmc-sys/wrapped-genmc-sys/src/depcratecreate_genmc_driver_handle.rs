// Generated macro for create_genmc_driver_handle (function)
macro_rules! Depcratecreate_genmc_driver_handle {
() => {
// Module: crate
// Provides: {"create_genmc_driver_handle"}
// Dependencies: {}
pub fn create_genmc_driver_handle (params : & GenmcParams , genmc_log_level : LogLevel , do_estimation : bool ,) -> UniquePtr < MiriGenmcShim > { assert_eq ! (& genmc_log_level , GENMC_LOG_LEVEL . get_or_init (|| { unsafe { set_log_level_raw (genmc_log_level) } ; genmc_log_level }) , "Attempt to change the GenMC log level after it was already set") ; unsafe { MiriGenmcShim :: create_handle (params , do_estimation) } }
};
}
