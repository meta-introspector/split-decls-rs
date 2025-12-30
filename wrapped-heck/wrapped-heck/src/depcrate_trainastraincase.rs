// Generated macro for AsTrainCase (struct)
macro_rules! Depcrate_trainAsTrainCase {
() => {
// Module: crate::train
// Provides: {"AsTrainCase"}
// Dependencies: {}
# [doc = " This wrapper performs a train case conversion in [`fmt::Display`]."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```"] # [doc = " use heck::AsTrainCase;"] # [doc = ""] # [doc = " let sentence = \"We are going to inherit the earth.\";"] # [doc = " assert_eq!(format!(\"{}\", AsTrainCase(sentence)), \"We-Are-Going-To-Inherit-The-Earth\");"] # [doc = " ```"] pub struct AsTrainCase < T : AsRef < str > > (pub T) ;
};
}
