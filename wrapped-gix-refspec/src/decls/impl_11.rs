macro_rules! deps {
    () => {
        Mode!();
        RefSpec!();
        RefSpecRef!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [doc = " Conversion. Use the [`RefSpecRef`][RefSpec::to_ref()] type for more usage options."] impl RefSpec { # [doc = " Return ourselves as reference type."] pub fn to_ref (& self) -> RefSpecRef < '_ > { RefSpecRef { mode : self . mode , op : self . op , src : self . src . as_ref () . map (AsRef :: as_ref) , dst : self . dst . as_ref () . map (AsRef :: as_ref) , } } # [doc = " Return true if the spec stats with a `+` and thus forces setting the reference."] pub fn allow_non_fast_forward (& self) -> bool { matches ! (self . mode , Mode :: Force) } }
    };
}

impl_11!()