// Generated macro for Value (struct)
macro_rules! Depcrate_ir_entitiesValue {
() => {
// Module: crate::ir::entities
// Provides: {"Value"}
// Dependencies: {}
# [doc = " An opaque reference to an SSA value."] # [doc = ""] # [doc = " You can get a constant `Value` from the following"] # [doc = " [`InstBuilder`](super::InstBuilder) instructions:"] # [doc = ""] # [doc = " - [`iconst`](super::InstBuilder::iconst) for integer constants"] # [doc = " - [`f16const`](super::InstBuilder::f16const) for 16-bit float constants"] # [doc = " - [`f32const`](super::InstBuilder::f32const) for 32-bit float constants"] # [doc = " - [`f64const`](super::InstBuilder::f64const) for 64-bit float constants"] # [doc = " - [`f128const`](super::InstBuilder::f128const) for 128-bit float constants"] # [doc = " - [`vconst`](super::InstBuilder::vconst) for vector constants"] # [doc = " - [`null`](super::InstBuilder::null) for null reference constants"] # [doc = ""] # [doc = " Any `InstBuilder` instruction that has an output will also return a `Value`."] # [doc = ""] # [doc = " While the order is stable, it is arbitrary."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Value (u32) ;
};
}
