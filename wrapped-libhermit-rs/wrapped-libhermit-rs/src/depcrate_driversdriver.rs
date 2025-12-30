// Generated macro for Driver (trait)
macro_rules! Depcrate_driversDriver {
() => {
// Module: crate::drivers
// Provides: {"Driver"}
// Dependencies: {}
# [doc = " A trait to determine general driver information"] # [allow (dead_code)] pub (crate) trait Driver { # [doc = " Returns the interrupt number of the device"] fn get_interrupt_number (& self) -> InterruptLine ; # [doc = " Returns the device driver name"] fn get_name (& self) -> & 'static str ; }
};
}
