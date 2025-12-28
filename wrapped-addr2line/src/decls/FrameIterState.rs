macro_rules! deps {
    () => {
        FrameIterFrames!();
        Location!();
    };
}

macro_rules! FrameIterState {
    () => {
        deps!();
        enum FrameIterState < 'ctx , R > where R : gimli :: Reader , { Empty , Location (Option < Location < 'ctx > >) , Frames (FrameIterFrames < 'ctx , R >) , }
    };
}

FrameIterState!();