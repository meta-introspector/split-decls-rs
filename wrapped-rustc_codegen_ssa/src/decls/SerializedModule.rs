macro_rules! deps {
    () => {
        ModuleBufferMethods!();
    };
}

macro_rules! SerializedModule {
    () => {
        deps!();
        pub enum SerializedModule < M : ModuleBufferMethods > { Local (M) , FromRlib (Vec < u8 >) , FromUncompressedFile (Mmap) , }
    };
}

SerializedModule!();