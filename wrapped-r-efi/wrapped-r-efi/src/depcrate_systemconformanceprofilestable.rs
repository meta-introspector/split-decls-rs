// Generated macro for ConformanceProfilesTable (struct)
macro_rules! Depcrate_systemConformanceProfilesTable {
() => {
// Module: crate::system
// Provides: {"ConformanceProfilesTable"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct ConformanceProfilesTable < const N : usize = 0 > { pub version : u16 , pub number_of_profiles : u16 , pub conformance_profiles : [crate :: base :: Guid ; N] , }
};
}
