// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_basic_validation () { let service = MockDataAccess :: default () ; let result = validate_split ("mock/dataset" , "default" , "train" , service) ; assert ! (result . is_ok ()) ; let (validation_result , progress) = result . unwrap () ; assert_eq ! (progress , 1.0) ; assert ! (validation_result . has_any_capability ()) ; } # [test] fn test_entity_identifier () { let entity = EntityIdentifier :: new_split ("test" . to_string () , "config" . to_string () , "split" . to_string ()) ; assert_eq ! (entity . dataset , "test") ; assert_eq ! (entity . config , Some ("config" . to_string ())) ; assert_eq ! (entity . split , Some ("split" . to_string ())) ; assert_eq ! (entity . infer_level () , ValidationLevel :: Split) ; assert_eq ! (entity . to_string () , "test/config/split") ; } # [test] fn test_validation_result () { let mut result1 = validator :: ValidationResult { viewer : true , preview : false , search : true , filter : false , statistics : true , } ; let result2 = validator :: ValidationResult { viewer : false , preview : true , search : false , filter : true , statistics : false , } ; result1 . merge (& result2) ; assert ! (result1 . viewer) ; assert ! (result1 . preview) ; assert ! (result1 . search) ; assert ! (result1 . filter) ; assert ! (result1 . statistics) ; assert_eq ! (result1 . capability_count () , 5) ; } }
};
}
