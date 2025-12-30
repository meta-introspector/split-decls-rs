// Generated macro for impl_410 (impl)
macro_rules! Depcrate_driverimpl_410 {
() => {
// Module: crate::driver
// Provides: {"impl_410"}
// Dependencies: {}
impl cranelift_codegen :: timing :: Profiler for MeasuremeProfiler { fn start_pass (& self , pass : cranelift_codegen :: timing :: Pass) -> Box < dyn std :: any :: Any > { let mut timing_guard = Box :: new (TimingGuard { profiler : std :: mem :: ManuallyDrop :: new (self . 0 . clone ()) , inner : None , }) ; timing_guard . inner = Some (unsafe { & * (& * timing_guard . profiler as & SelfProfilerRef as * const SelfProfilerRef) } . generic_activity (pass . description ()) ,) ; timing_guard } }
};
}
