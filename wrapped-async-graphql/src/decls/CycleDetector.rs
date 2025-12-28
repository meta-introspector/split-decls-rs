macro_rules! deps {
    () => {
        RuleError!();
    };
}

macro_rules! CycleDetector {
    () => {
        deps!();
        struct CycleDetector < 'a > { visited : HashSet < & 'a str > , spreads : & 'a HashMap < & 'a str , Vec < (& 'a str , Pos) > > , path_indices : HashMap < & 'a str , usize > , errors : Vec < RuleError > , }
    };
}

CycleDetector!()