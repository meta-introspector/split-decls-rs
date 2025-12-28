macro_rules! gb {
    () => {
        # [doc = " Converts a quantity of gigabytes to bytes."] pub fn gb < V : Into < u64 > > (size : V) -> u64 { size . into () * GB }
    };
}

gb!();