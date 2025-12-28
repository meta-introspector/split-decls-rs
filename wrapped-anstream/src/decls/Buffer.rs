macro_rules! Buffer {
    () => {
        # [doc = " In-memory [`RawStream`][crate::stream::RawStream]"] # [derive (Clone , Default , Debug , PartialEq , Eq)] # [deprecated (since = "0.6.2" , note = "Use Vec")] # [doc (hidden)] pub struct Buffer (Vec < u8 >) ;
    };
}

Buffer!();