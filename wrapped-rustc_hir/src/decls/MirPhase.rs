macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! MirPhase {
    () => {
        deps!();
        # [derive (Clone , Copy , Decodable , Debug , Encodable , PartialEq)] # [derive (HashStable_Generic , PrintAttribute)] pub enum MirPhase { Initial , PostCleanup , Optimized , }
    };
}

MirPhase!()