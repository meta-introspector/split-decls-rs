// Generated macro for test (module)
macro_rules! Depcrate_provider_fields_componentstest {
() => {
// Module: crate::provider::fields::components
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; type Symbol = FieldSymbol ; type Length = FieldLength ; # [test] fn test_component_bag_to_vec_field () { let bag = Bag { year : Some (Year :: Numeric) , month : Some (Month :: Long) , week : None , day : Some (Day :: NumericDayOfMonth) , hour : Some (Numeric :: Numeric) , minute : Some (Numeric :: Numeric) , second : Some (Numeric :: Numeric) , subsecond : Some (SubsecondDigits :: S3) , .. Default :: default () } ; assert_eq ! (bag . to_vec_fields (HourCycle :: H23) , [(Symbol :: Year (fields :: Year :: Calendar) , Length :: One) . into () , (Symbol :: Month (fields :: Month :: Format) , Length :: Four) . into () , (Symbol :: Day (fields :: Day :: DayOfMonth) , Length :: One) . into () , (Symbol :: Hour (fields :: Hour :: H23) , Length :: One) . into () , (Symbol :: Minute , Length :: One) . into () , (Symbol :: DecimalSecond (fields :: DecimalSecond :: Subsecond3) , Length :: One) . into () ,]) ; } # [test] fn test_component_bag_to_vec_field2 () { let bag = Bag { year : Some (Year :: Numeric) , month : Some (Month :: TwoDigit) , day : Some (Day :: NumericDayOfMonth) , .. Default :: default () } ; assert_eq ! (bag . to_vec_fields (HourCycle :: H23) , [(Symbol :: Year (fields :: Year :: Calendar) , Length :: One) . into () , (Symbol :: Month (fields :: Month :: Format) , Length :: Two) . into () , (Symbol :: Day (fields :: Day :: DayOfMonth) , Length :: One) . into () ,]) ; } }
};
}
