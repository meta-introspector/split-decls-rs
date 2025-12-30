// Generated macro for Drive (struct)
macro_rules! Depcrate_future_extDrive {
() => {
// Module: crate::future_ext
// Provides: {"Drive"}
// Dependencies: {}
# [doc = " Drive a future to completion while also polling the driver"] # [doc = ""] # [doc = " This is useful for H2 futures that also require the connection to be polled."] pub struct Drive < 'a , T , U > { driver : & 'a mut T , future : Wakened < U > , }
};
}
