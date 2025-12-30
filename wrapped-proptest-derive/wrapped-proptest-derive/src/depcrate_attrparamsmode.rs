// Generated macro for ParamsMode (enum)
macro_rules! Depcrate_attrParamsMode {
() => {
// Module: crate::attr
// Provides: {"ParamsMode"}
// Dependencies: {}
# [doc = " The mode for the associated item `Parameters` to use."] # [derive (Clone)] pub enum ParamsMode { # [doc = " Nothing has been specified. The children are now free to"] # [doc = " specify their parameters, and if nothing is specified, then"] # [doc = " `<X as Arbitrary>::Parameters` will be used for a type `X`."] Passthrough , # [doc = " We've been ordered to use the Default value of"] # [doc = " `<X as Arbitrary>::Parameters` for some field where applicable."] # [doc = " For the top level item, this means that `Parameters` will be"] # [doc = " the unit type. For children, it means that this child should"] # [doc = " not count towards the product type that is being built up."] Default , # [doc = " An explicit type has been specified on some item."] # [doc = " If the top level item has this specified on it, this means"] # [doc = " that `Parameters` will have the given type."] # [doc = " If it is specified on a child of the top level item, this"] # [doc = " entails that the given type will be added to the resultant"] # [doc = " product type."] Specified (Type) , }
};
}
