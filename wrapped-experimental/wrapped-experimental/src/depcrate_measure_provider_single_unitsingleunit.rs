// Generated macro for SingleUnit (struct)
macro_rules! Depcrate_measure_provider_single_unitSingleUnit {
() => {
// Module: crate::measure::provider::single_unit
// Provides: {"SingleUnit"}
// Dependencies: {}
# [doc = " Represents a single unit in a measure unit."] # [doc = " For example, the MeasureUnit `kilometer-per-square-second` contains two single units:"] # [doc = "    1. `kilometer` with power 1 and prefix 3 with base 10."] # [doc = "    2. `second` with power -2 and prefix power equal to 0."] # [zerovec :: make_ule (SingleUnitULE)] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq , Default)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: measure :: provider :: single_unit))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct SingleUnit { # [doc = " The power of the unit."] pub power : i8 , # [doc = " The si base of the unit."] pub si_prefix : SiPrefix , # [doc = " The id of the unit."] pub unit_id : UnitID , }
};
}
