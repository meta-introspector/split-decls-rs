// Generated macro for CONSOLE (static)
macro_rules! Depcrate_consoleCONSOLE {
() => {
// Module: crate::console
// Provides: {"CONSOLE"}
// Dependencies: {}
pub (crate) static CONSOLE : Lazy < InterruptTicketMutex < Console > > = Lazy :: new (| | { crate :: CoreLocal :: install () ; # [cfg (not (target_arch = "riscv64"))] if crate :: env :: is_uhyve () { InterruptTicketMutex :: new (Console :: new (IoDevice :: Uhyve (UhyveSerial :: new ()))) } else { InterruptTicketMutex :: new (Console :: new (IoDevice :: Uart (SerialDevice :: new ()))) } # [cfg (target_arch = "riscv64")] InterruptTicketMutex :: new (Console :: new (IoDevice :: Uart (SerialDevice :: new ()))) }) ;
};
}
