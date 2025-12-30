// Generated macro for impl_852 (impl)
macro_rules! Depcrate_rc_test_objectimpl_852 {
() => {
// Module: crate::rc::test_object
// Provides: {"impl_852"}
// Dependencies: {}
impl Drop for RcTestObject { fn drop (& mut self) { TEST_DATA . with (| data | data . borrow_mut () . drop += 1) ; } }
};
}
