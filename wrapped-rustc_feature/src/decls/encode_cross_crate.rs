macro_rules! encode_cross_crate {
    () => {
        # [doc = " Whether this builtin attribute is encoded cross crate."] # [doc = " This means it can be used cross crate."] pub fn encode_cross_crate (name : Symbol) -> bool { if let Some (attr) = BUILTIN_ATTRIBUTE_MAP . get (& name) { attr . encode_cross_crate == EncodeCrossCrate :: Yes } else { true } }
    };
}

encode_cross_crate!();