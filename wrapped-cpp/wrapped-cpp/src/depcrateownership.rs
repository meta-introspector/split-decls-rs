// Generated macro for Ownership (enum)
macro_rules! DepcrateOwnership {
() => {
// Module: crate
// Provides: {"Ownership"}
// Dependencies: {}
# [derive (Default , Debug , Clone , Copy)] pub enum Ownership { # [doc = " Generated types will be composed entirely of owning fields, regardless"] # [doc = " of whether they are used as parameters to imports or not."] # [default] Owning , # [doc = " Generated types used as parameters to imports will be \"deeply"] # [doc = " borrowing\", i.e. contain references rather than owned values when"] # [doc = " applicable."] CoarseBorrowing , # [doc = " Generated types used as parameters to imports will be \"deeply"] # [doc = " borrowing\", i.e. contain references rather than owned values"] # [doc = " for all fields that are not resources, which will be owning."] FineBorrowing , }
};
}
