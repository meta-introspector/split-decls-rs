// Generated macro for Ownership (enum)
macro_rules! DepcrateOwnership {
() => {
// Module: crate
// Provides: {"Ownership"}
// Dependencies: {}
# [derive (Default , Debug , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize) , serde (rename_all = "kebab-case"))] pub enum Ownership { # [doc = " Generated types will be composed entirely of owning fields, regardless"] # [doc = " of whether they are used as parameters to imports or not."] # [default] Owning , # [doc = " Generated types used as parameters to imports will be \"deeply"] # [doc = " borrowing\", i.e. contain references rather than owned values when"] # [doc = " applicable."] Borrowing { # [doc = " Whether or not to generate \"duplicate\" type definitions for a single"] # [doc = " WIT type if necessary, for example if it's used as both an import"] # [doc = " and an export, or if it's used both as a parameter to an import and"] # [doc = " a return value from an import."] duplicate_if_necessary : bool , } , }
};
}
