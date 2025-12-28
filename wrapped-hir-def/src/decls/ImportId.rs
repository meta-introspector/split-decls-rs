macro_rules! deps {
    () => {
        UseTree!();
    };
}

macro_rules! ImportId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Ord , PartialOrd)] pub struct ImportId { pub use_ : UseId , pub idx : Idx < ast :: UseTree > , }
    };
}

ImportId!();