macro_rules! deps {
    () => {
        PageTag!();
    };
}

macro_rules! split_streams {
    () => {
        deps!();
        # [doc = " This function reconstructs the individual data streams from their paged"] # [doc = " version."] # [doc = ""] # [doc = " For example, if `E` denotes the page header of an events page, `S` denotes"] # [doc = " the header of a string data page, and lower case letters denote page"] # [doc = " contents then a paged stream could look like:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " s = Eabcd_Sopq_Eef_Eghi_Srst"] # [doc = " ```"] # [doc = ""] # [doc = " and `split_streams` would result in the following set of streams:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " split_streams(s) = {"] # [doc = "     events: [abcdefghi],"] # [doc = "     string_data: [opqrst],"] # [doc = " }"] # [doc = " ```"] pub fn split_streams (paged_data : & [u8]) -> FxHashMap < PageTag , Vec < u8 > > { let mut result : FxHashMap < PageTag , Vec < u8 > > = FxHashMap :: default () ; let mut pos = 0 ; while pos < paged_data . len () { let tag = TryInto :: try_into (paged_data [pos]) . unwrap () ; let page_size = u32 :: from_le_bytes (paged_data [pos + 1 .. pos + 5] . try_into () . unwrap ()) as usize ; assert ! (page_size > 0) ; result . entry (tag) . or_default () . extend_from_slice (& paged_data [pos + 5 .. pos + 5 + page_size]) ; pos += page_size + 5 ; } result }
    };
}

split_streams!()