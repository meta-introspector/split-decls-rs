// Generated macro for DropGlue (enum)
macro_rules! Depcrate_dropDropGlue {
() => {
// Module: crate::drop
// Provides: {"DropGlue"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub enum DropGlue { None , # [doc = " May have a drop glue if some type parameter has it."] # [doc = ""] # [doc = " For the compiler this is considered as a positive result, IDE distinguishes this from \"yes\"."] DependOnParams , HasDropGlue , }
};
}
