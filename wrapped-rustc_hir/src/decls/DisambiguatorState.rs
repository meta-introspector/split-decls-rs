macro_rules! deps {
    () => {
        DefPathData!();
    };
}

macro_rules! DisambiguatorState {
    () => {
        deps!();
        # [derive (Debug)] pub struct DisambiguatorState { next : UnordMap < (LocalDefId , DefPathData) , u32 > , }
    };
}

DisambiguatorState!()