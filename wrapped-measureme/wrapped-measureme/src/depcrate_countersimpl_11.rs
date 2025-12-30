// Generated macro for impl_11 (impl)
macro_rules! Depcrate_countersimpl_11 {
() => {
// Module: crate::counters
// Provides: {"impl_11"}
// Dependencies: {}
impl Instructions { const NAME : & 'static str = "instructions:u" ; pub fn new () -> Result < Self , Box < dyn Error + Send + Sync > > { let model = hw :: CpuModel :: detect () ? ; let instructions = hw :: Counter :: new (& model , HwCounterType :: Instructions) ? ; let start = instructions . read () ; Ok (Instructions { instructions , start , }) } # [inline] fn since_start (& self) -> u64 { self . instructions . read () . wrapping_sub (self . start) } }
};
}
