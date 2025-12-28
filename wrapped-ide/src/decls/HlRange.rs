macro_rules! HlRange {
    () => {
        # [derive (Debug , Clone , Copy)] pub struct HlRange { pub range : TextRange , pub highlight : Highlight , pub binding_hash : Option < u64 > , }
    };
}

HlRange!();