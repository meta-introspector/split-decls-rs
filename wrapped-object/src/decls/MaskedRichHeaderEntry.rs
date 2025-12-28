macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! MaskedRichHeaderEntry {
    () => {
        deps!();
        # [doc = " A PE rich header entry."] # [doc = ""] # [doc = " Rich headers have no official documentation, but have been heavily"] # [doc = " reversed-engineered and documented in the wild, e.g.:"] # [doc = " * `http://www.ntcore.com/files/richsign.htm`"] # [doc = " * `https://www.researchgate.net/figure/Structure-of-the-Rich-Header_fig1_318145388`"] # [doc = ""] # [doc = " This data is \"masked\", i.e. XORed with a checksum derived from the file data."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct MaskedRichHeaderEntry { pub masked_comp_id : U32 < LE > , pub masked_count : U32 < LE > , }
    };
}

MaskedRichHeaderEntry!();