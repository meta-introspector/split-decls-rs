// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_i2cErrorKind {
() => {
// Module: crate::i2c
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " I2C error kind."] # [doc = ""] # [doc = " This represents a common set of I2C operation errors. HAL implementations are"] # [doc = " free to define more specific or additional error types. However, by providing"] # [doc = " a mapping to these common I2C errors, generic code can still react to them."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] # [non_exhaustive] pub enum ErrorKind { # [doc = " Bus error occurred. e.g. A START or a STOP condition is detected and is not"] # [doc = " located after a multiple of 9 SCL clock pulses."] Bus , # [doc = " The arbitration was lost, e.g. electrical problems with the clock signal."] ArbitrationLoss , # [doc = " A bus operation was not acknowledged, e.g. due to the addressed device not"] # [doc = " being available on the bus or the device not being ready to process requests"] # [doc = " at the moment."] NoAcknowledge (NoAcknowledgeSource) , # [doc = " The peripheral receive buffer was overrun."] Overrun , # [doc = " A different error occurred. The original error may contain more information."] Other , }
};
}
