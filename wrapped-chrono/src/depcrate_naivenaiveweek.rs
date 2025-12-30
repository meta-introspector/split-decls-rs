// Generated macro for NaiveWeek (struct)
macro_rules! Depcrate_naiveNaiveWeek {
() => {
// Module: crate::naive
// Provides: {"NaiveWeek"}
// Dependencies: {}
# [doc = " A week represented by a [`NaiveDate`] and a [`Weekday`] which is the first"] # [doc = " day of the week."] # [derive (Clone , Copy , Debug , Eq)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct NaiveWeek { date : NaiveDate , start : Weekday , }
};
}
