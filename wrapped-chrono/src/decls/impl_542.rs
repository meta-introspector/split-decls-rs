macro_rules! deps {
    () => {
        FixedOffset!();
    };
}

macro_rules! impl_542 {
    () => {
        deps!();
        # [cfg (all (feature = "arbitrary" , feature = "std"))] impl arbitrary :: Arbitrary < '_ > for FixedOffset { fn arbitrary (u : & mut arbitrary :: Unstructured) -> arbitrary :: Result < FixedOffset > { let secs = u . int_in_range (- 86_399 ..= 86_399) ? ; let fixed_offset = FixedOffset :: east_opt (secs) . expect ("Could not generate a valid chrono::FixedOffset. It looks like implementation of Arbitrary for FixedOffset is erroneous.") ; Ok (fixed_offset) } }
    };
}

impl_542!();