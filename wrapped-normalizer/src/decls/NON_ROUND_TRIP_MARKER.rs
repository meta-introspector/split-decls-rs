macro_rules! NON_ROUND_TRIP_MARKER {
    () => {
        # [doc = " Marker that the decomposition does not round trip via NFC."] # [doc = ""] # [doc = " See trie-value-format.md"] const NON_ROUND_TRIP_MARKER : u32 = 1 << 30 ;
    };
}

NON_ROUND_TRIP_MARKER!();