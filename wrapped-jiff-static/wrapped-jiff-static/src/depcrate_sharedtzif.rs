// Generated macro for Tzif (struct)
macro_rules! Depcrate_sharedTzif {
() => {
// Module: crate::shared
// Provides: {"Tzif"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct Tzif < STR , ABBREV , TYPES , TIMESTAMPS , STARTS , ENDS , INFOS > { pub fixed : TzifFixed < STR , ABBREV > , pub types : TYPES , pub transitions : TzifTransitions < TIMESTAMPS , STARTS , ENDS , INFOS > , }
};
}
