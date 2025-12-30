// Generated macro for impl_45 (impl)
macro_rules! Depcrate_i2c_rcimpl_45 {
() => {
// Module: crate::i2c::rc
// Provides: {"impl_45"}
// Dependencies: {}
impl < Bus > RcDevice < Bus > { # [doc = " Creates a new `RcDevice`."] # [doc = ""] # [doc = " This function does not increment the reference count for the bus:"] # [doc = " you will need to call `Rc::clone(&bus)` if you only have a `&Rc<RefCell<Bus>>`."] # [inline] pub fn new (bus : Rc < RefCell < Bus > >) -> Self { Self { bus } } }
};
}
