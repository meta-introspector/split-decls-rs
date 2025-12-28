macro_rules! deps {
    () => {
        Engine!();
    };
}

macro_rules! DecodePaddingMode {
    () => {
        deps!();
        # [doc = " Controls how pad bytes are handled when decoding."] # [doc = ""] # [doc = " Each [Engine] must support at least the behavior indicated by"] # [doc = " [`DecodePaddingMode::RequireCanonical`], and may support other modes."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DecodePaddingMode { # [doc = " Canonical padding is allowed, but any fewer padding bytes than that is also allowed."] Indifferent , # [doc = " Padding must be canonical (0, 1, or 2 `=` as needed to produce a 4 byte suffix)."] RequireCanonical , # [doc = " Padding must be absent -- for when you want predictable padding, without any wasted bytes."] RequireNone , }
    };
}

DecodePaddingMode!()