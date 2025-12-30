// Generated macro for SharedState (struct)
macro_rules! Depcrate_submoduleSharedState {
() => {
// Module: crate::submodule
// Provides: {"SharedState"}
// Dependencies: {}
# [doc = " A platform maintaining state needed to interact with submodules, created by [`Repository::submodules()]."] pub (crate) struct SharedState < 'repo > { pub repo : & 'repo Repository , pub (crate) modules : ModulesSnapshot , is_active : RefCell < Option < IsActiveState > > , index : RefCell < Option < IndexPersistedOrInMemory > > , }
};
}
