macro_rules! deps {
    () => {
        ItemTree!();
    };
}

macro_rules! TreeId {
    () => {
        deps!();
        # [doc = " Identifies a particular [`ItemTree`]."] # [derive (Debug , PartialEq , Eq , Clone , Copy , Hash)] pub struct TreeId { file : HirFileId , block : Option < BlockId > , }
    };
}

TreeId!()