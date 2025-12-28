macro_rules! deps {
    () => {
        DisambiguatorState!();
        DefPathData!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl DisambiguatorState { pub fn new () -> Self { Self { next : Default :: default () } } # [doc = " Creates a `DisambiguatorState` where the next allocated `(LocalDefId, DefPathData)` pair"] # [doc = " will have `index` as the disambiguator."] pub fn with (def_id : LocalDefId , data : DefPathData , index : u32) -> Self { let mut this = Self :: new () ; this . next . insert ((def_id , data) , index) ; this } }
    };
}

impl_86!()