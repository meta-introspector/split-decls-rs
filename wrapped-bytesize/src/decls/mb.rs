macro_rules! mb {
    () => {
        # [doc = " Converts a quantity of megabytes to bytes."] pub fn mb < V : Into < u64 > > (size : V) -> u64 { size . into () * MB }
    };
}

mb!();