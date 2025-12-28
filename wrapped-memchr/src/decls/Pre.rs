macro_rules! deps {
    () => {
        Prefilter!();
        PrefilterState!();
    };
}

macro_rules! Pre {
    () => {
        deps!();
        # [doc = " A combination of prefilter effectiveness state and the prefilter itself."] # [derive (Debug)] pub (crate) struct Pre < 'a > { # [doc = " State that tracks the effectiveness of a prefilter."] prestate : & 'a mut PrefilterState , # [doc = " The actual prefilter."] prestrat : & 'a Prefilter , }
    };
}

Pre!()