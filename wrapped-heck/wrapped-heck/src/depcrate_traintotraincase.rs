// Generated macro for ToTrainCase (trait)
macro_rules! Depcrate_trainToTrainCase {
() => {
// Module: crate::train
// Provides: {"ToTrainCase"}
// Dependencies: {}
# [doc = " This trait defines a train case conversion."] # [doc = ""] # [doc = " In Train-Case, word boundaries are indicated by hyphens and words start"] # [doc = " with Capital Letters."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use heck::ToTrainCase;"] # [doc = ""] # [doc = " let sentence = \"We are going to inherit the earth.\";"] # [doc = " assert_eq!(sentence.to_train_case(), \"We-Are-Going-To-Inherit-The-Earth\");"] # [doc = " ```"] pub trait ToTrainCase : ToOwned { # [doc = " Convert this type to Train-Case."] fn to_train_case (& self) -> Self :: Owned ; }
};
}
