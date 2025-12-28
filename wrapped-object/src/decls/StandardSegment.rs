macro_rules! StandardSegment {
    () => {
        # [doc = " A standard segment kind."] # [allow (missing_docs)] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [non_exhaustive] pub enum StandardSegment { Text , Data , Debug , }
    };
}

StandardSegment!();