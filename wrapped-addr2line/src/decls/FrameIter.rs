macro_rules! deps {
    () => {
        FrameIterState!();
    };
}

macro_rules! FrameIter {
    () => {
        deps!();
        # [doc = " An iterator over function frames."] pub struct FrameIter < 'ctx , R > (FrameIterState < 'ctx , R >) where R : gimli :: Reader ;
    };
}

FrameIter!();