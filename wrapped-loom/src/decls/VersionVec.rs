macro_rules! VersionVec {
    () => {
        # [derive (Debug , Copy , Clone , Eq , PartialEq)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (crate) struct VersionVec { versions : [u16 ; MAX_THREADS] , }
    };
}

VersionVec!()