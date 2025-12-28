macro_rules! deps {
    () => {
        LocalModuleId!();
    };
}

macro_rules! BlockRelativeModuleId {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone , Copy)] struct BlockRelativeModuleId { block : Option < BlockId > , local_id : LocalModuleId , }
    };
}

BlockRelativeModuleId!();