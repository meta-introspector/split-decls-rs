macro_rules! deps {
    () => {
        UseTree!();
    };
}

macro_rules! GlobId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Ord , PartialOrd)] pub struct GlobId { pub use_ : UseId , pub idx : Idx < ast :: UseTree > , }
    };
}

GlobId!();