macro_rules! eb {
    () => {
        # [doc = " Converts a quantity of exabytes to bytes."] pub fn eb < V : Into < u64 > > (size : V) -> u64 { size . into () * EB }
    };
}

eb!()