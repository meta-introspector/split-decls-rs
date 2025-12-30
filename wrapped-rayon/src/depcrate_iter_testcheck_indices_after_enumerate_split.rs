// Generated macro for check_indices_after_enumerate_split (function)
macro_rules! Depcrate_iter_testcheck_indices_after_enumerate_split {
() => {
// Module: crate::iter::test
// Provides: {"check_indices_after_enumerate_split"}
// Dependencies: {}
# [test] fn check_indices_after_enumerate_split () { let a : Vec < i32 > = (0 .. 1024) . collect () ; a . par_iter () . enumerate () . with_producer (WithProducer) ; struct WithProducer ; impl < 'a > ProducerCallback < (usize , & 'a i32) > for WithProducer { type Output = () ; fn callback < P > (self , producer : P) where P : Producer < Item = (usize , & 'a i32) > , { let (a , b) = producer . split_at (512) ; for ((index , value) , trusted_index) in a . into_iter () . zip (0 ..) { assert_eq ! (index , trusted_index) ; assert_eq ! (index , * value as usize) ; } for ((index , value) , trusted_index) in b . into_iter () . zip (512 ..) { assert_eq ! (index , trusted_index) ; assert_eq ! (index , * value as usize) ; } } } }
};
}
