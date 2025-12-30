// Generated macro for check_de_any_number (function)
macro_rules! Depcrate_de_testscheck_de_any_number {
() => {
// Module: crate::de::tests
// Provides: {"check_de_any_number"}
// Dependencies: {}
fn check_de_any_number < T : Copy + PartialEq + core :: fmt :: Debug + Into < Number > + serde :: de :: DeserializeOwned , > (s : & str , cmp : T ,) { let mut parser = Parser :: new (s) . unwrap () ; let number = parser . any_number () . unwrap () ; assert_eq ! (number , Number :: new (cmp)) ; assert_eq ! (Number :: new (super :: from_str ::< T > (s) . unwrap ()) , Number :: new (cmp)) ; }
};
}
