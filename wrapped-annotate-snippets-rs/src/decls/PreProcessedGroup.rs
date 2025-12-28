macro_rules! deps {
    () => {
        Group!();
        PreProcessedElement!();
    };
}

macro_rules! PreProcessedGroup {
    () => {
        deps!();
        struct PreProcessedGroup < 'a > { group : & 'a Group < 'a > , elements : Vec < PreProcessedElement < 'a > > , primary_path : Option < & 'a Cow < 'a , str > > , max_depth : usize , }
    };
}

PreProcessedGroup!();