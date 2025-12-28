macro_rules! deps {
    () => {
        FasNode!();
        FasNodeIndex!();
    };
}

macro_rules! FasNodeContainer {
    () => {
        deps!();
        # [derive (Debug)] struct FasNodeContainer { nodes : Vec < LinkedListEntry < FasNode , FasNodeIndex > > , }
    };
}

FasNodeContainer!()