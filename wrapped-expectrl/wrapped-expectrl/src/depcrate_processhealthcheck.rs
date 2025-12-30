// Generated macro for Healthcheck (trait)
macro_rules! Depcrate_processHealthcheck {
() => {
// Module: crate::process
// Provides: {"Healthcheck"}
// Dependencies: {}
# [allow (clippy :: wrong_self_convention)] # [doc = " Healthcheck represents a check by which we can determine if a spawned process is still alive."] pub trait Healthcheck { # [doc = " A status healthcheck can return."] type Status ; # [doc = " The function returns a status of a process if it still alive and it can operate."] fn get_status (& self) -> Result < Self :: Status > ; # [doc = " The function returns a status of a process if it still alive and it can operate."] fn is_alive (& self) -> Result < bool > ; }
};
}
