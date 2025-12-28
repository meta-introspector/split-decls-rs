macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! band {
    () => {
        deps!();
        # [doc = ""] pub mod band { # [doc = " The error used in [`PacketLineRef::decode_band()`][super::PacketLineRef::decode_band()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("attempt to decode a non-side channel line or input was malformed: {band_id}")] InvalidSideBand { band_id : u8 } , # [error ("attempt to decode a non-data line into a side-channel band")] NonDataLine , } }
    };
}

band!();