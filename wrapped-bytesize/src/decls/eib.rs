macro_rules! eib {
    () => {
        # [doc = " Converts a quantity of exbibytes to bytes."] pub fn eib < V : Into < u64 > > (size : V) -> u64 { size . into () * EIB }
    };
}

eib!()