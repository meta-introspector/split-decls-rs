// Generated macro for UniqueCrateData (struct)
macro_rules! Depcrate_inputUniqueCrateData {
() => {
// Module: crate::input
// Provides: {"UniqueCrateData"}
// Dependencies: {}
# [doc = " The crate data from which we derive the `Crate`."] # [doc = ""] # [doc = " We want this to contain as little data as possible, because if it contains dependencies and"] # [doc = " something changes, this crate and all of its dependencies ids are invalidated, which causes"] # [doc = " pretty much everything to be recomputed. If the crate id is not invalidated, only this crate's"] # [doc = " information needs to be recomputed."] # [doc = ""] # [doc = " *Most* different crates have different root files (actually, pretty much all of them)."] # [doc = " Still, it is possible to have crates distinguished by other factors (e.g. dependencies)."] # [doc = " So we store only the root file - unless we find that this crate has the same root file as"] # [doc = " another crate, in which case we store all data for one of them (if one is a dependency of"] # [doc = " the other, we store for it, because it has more dependencies to be invalidated)."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct UniqueCrateData { root_file_id : FileId , disambiguator : Option < Box < (BuiltCrateData , HashableCfgOptions) > > , }
};
}
