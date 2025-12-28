macro_rules! deps {
    () => {
        FluentType!();
        FluentNumber!();
    };
}

macro_rules! FluentValue {
    () => {
        deps!();
        # [doc = " The `FluentValue` enum represents values which can be formatted to a String."] # [doc = ""] # [doc = " Those values are either passed as arguments to [`FluentBundle::format_pattern`] or"] # [doc = " produced by functions, or generated in the process of pattern resolution."] # [doc = ""] # [doc = " [`FluentBundle::format_pattern`]: crate::bundle::FluentBundle::format_pattern"] # [derive (Debug)] pub enum FluentValue < 'source > { String (Cow < 'source , str >) , Number (FluentNumber) , Custom (Box < dyn FluentType + Send >) , None , Error , }
    };
}

FluentValue!();