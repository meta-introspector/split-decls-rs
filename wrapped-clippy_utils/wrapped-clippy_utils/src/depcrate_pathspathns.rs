// Generated macro for PathNS (enum)
macro_rules! Depcrate_pathsPathNS {
() => {
// Module: crate::paths
// Provides: {"PathNS"}
// Dependencies: {}
# [doc = " Specifies whether to resolve a path in the [`TypeNS`], [`ValueNS`], [`MacroNS`] or in an"] # [doc = " arbitrary namespace"] # [derive (Clone , Copy , PartialEq , Debug)] pub enum PathNS { Type , Value , Macro , # [doc = " Resolves to the name in the first available namespace, e.g. for `std::vec` this would return"] # [doc = " either the macro or the module but **not** both"] # [doc = ""] # [doc = " Must only be used when the specific resolution is unimportant such as in"] # [doc = " `missing_enforced_import_renames`"] Arbitrary , }
};
}
