macro_rules! deps {
    () => {
        NFA!();
    };
}

macro_rules! StateBuilderMatches {
    () => {
        deps!();
        # [doc = " A state builder that collects assertions and pattern IDs."] # [doc = ""] # [doc = " When collecting pattern IDs is finished, this can be converted into a"] # [doc = " builder that collects NFA state IDs."] # [derive (Clone)] pub (crate) struct StateBuilderMatches (Vec < u8 >) ;
    };
}

StateBuilderMatches!()