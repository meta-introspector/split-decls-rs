macro_rules! RawValueEmitter {
    () => {
        # [cfg (feature = "raw_value")] struct RawValueEmitter ;
    };
}

RawValueEmitter!();