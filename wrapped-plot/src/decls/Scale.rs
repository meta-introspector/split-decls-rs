macro_rules! deps {
    () => {
        Axis!();
    };
}

macro_rules! Scale {
    () => {
        deps!();
        # [doc = " Axis scale"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub enum Scale { Linear , Logarithmic , }
    };
}

Scale!()