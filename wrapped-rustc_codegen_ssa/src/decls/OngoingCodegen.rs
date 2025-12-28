macro_rules! deps {
    () => {
        CguMessage!();
        SharedEmitterMain!();
        CrateInfo!();
        ExtraBackendMethods!();
        Coordinator!();
    };
}

macro_rules! OngoingCodegen {
    () => {
        deps!();
        pub struct OngoingCodegen < B : ExtraBackendMethods > { pub backend : B , pub crate_info : CrateInfo , pub output_filenames : Arc < OutputFilenames > , pub coordinator : Coordinator < B > , pub codegen_worker_receive : Receiver < CguMessage > , pub shared_emitter_main : SharedEmitterMain , }
    };
}

OngoingCodegen!();