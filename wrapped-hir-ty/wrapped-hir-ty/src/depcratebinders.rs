// Generated macro for Binders (type)
macro_rules! DepcrateBinders {
() => {
// Module: crate
// Provides: {"Binders"}
// Dependencies: {}
# [doc = " Represents generic parameters and an item bound by them. When the item has parent, the binders"] # [doc = " also contain the generic parameters for its parent. See chalk's documentation for details."] # [doc = ""] # [doc = " One thing to keep in mind when working with `Binders` (and `Substitution`s, which represent"] # [doc = " generic arguments) in rust-analyzer is that the ordering within *is* significant - the generic"] # [doc = " parameters/arguments for an item MUST come before those for its parent. This is to facilitate"] # [doc = " the integration with chalk-solve, which mildly puts constraints as such. See #13335 for its"] # [doc = " motivation in detail."] pub type Binders < T > = chalk_ir :: Binders < T > ;
};
}
