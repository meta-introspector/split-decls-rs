macro_rules! LlvmSelfProfiler {
    () => {
        pub (crate) struct LlvmSelfProfiler < 'a > { profiler : Arc < SelfProfiler > , stack : Vec < TimingGuard < 'a > > , llvm_pass_event_kind : StringId , }
    };
}

LlvmSelfProfiler!()