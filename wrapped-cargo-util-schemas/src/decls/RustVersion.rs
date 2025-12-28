macro_rules! deps {
    () => {
        PartialVersion!();
    };
}

macro_rules! RustVersion {
    () => {
        deps!();
        # [derive (PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Debug , serde :: Serialize)] # [serde (transparent)] pub struct RustVersion (PartialVersion) ;
    };
}

RustVersion!()