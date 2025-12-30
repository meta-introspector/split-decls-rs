// Generated macro for strip_strategy (function)
macro_rules! Depcrate_property_test_utilsstrip_strategy {
() => {
// Module: crate::property_test::utils
// Provides: {"strip_strategy"}
// Dependencies: {}
fn strip_strategy (mut pat_ty : PatType) -> Argument { let (strategies , others) = pat_ty . attrs . into_iter () . partition (is_strategy) ; pat_ty . attrs = others ; let strategy = match & strategies [..] { [] => None , [s] => match & s . meta { Meta :: NameValue (name_value) => Some (name_value . value . clone ()) , _ => panic ! ("invalid strategies should be filtered by validate") , } , _ => panic ! ("multiple strategies should be filtered by validate") , } ; Argument { pat_ty , strategy } }
};
}
