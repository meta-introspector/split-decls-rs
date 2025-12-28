macro_rules! EncodeCrossCrate {
    () => {
        # [derive (PartialEq)] pub enum EncodeCrossCrate { Yes , No , }
    };
}

EncodeCrossCrate!()