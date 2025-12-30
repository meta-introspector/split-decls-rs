// Generated macro for ConsoleDevCfg (struct)
macro_rules! Depcrate_drivers_consoleConsoleDevCfg {
() => {
// Module: crate::drivers::console
// Provides: {"ConsoleDevCfg"}
// Dependencies: {}
# [doc = " A wrapper struct for the raw configuration structure."] # [doc = " Handling the right access to fields, as some are read-only"] # [doc = " for the driver."] pub (crate) struct ConsoleDevCfg { pub raw : VolatileRef < 'static , Config , ReadOnly > , pub dev_id : u16 , pub features : virtio :: console :: F , }
};
}
