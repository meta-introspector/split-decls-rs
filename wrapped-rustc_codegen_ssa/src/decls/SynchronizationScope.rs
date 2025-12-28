macro_rules! SynchronizationScope {
    () => {
        # [derive (Copy , Clone , Debug)] pub enum SynchronizationScope { SingleThread , CrossThread , }
    };
}

SynchronizationScope!()