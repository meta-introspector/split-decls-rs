// Generated macro for RcDevice (struct)
macro_rules! Depcrate_i2c_rcRcDevice {
() => {
// Module: crate::i2c::rc
// Provides: {"RcDevice"}
// Dependencies: {}
# [doc = " `Rc<RefCell<T>>`-based shared bus [`I2c`] implementation."] # [doc = " This is the reference-counting equivalent of [`RefCellDevice`](super::RefCellDevice)."] # [doc = ""] # [doc = " Sharing is implemented with a [`RefCell`] and ownership is managed by [`Rc`]."] # [doc = " Like [`RefCellDevice`](super::RefCellDevice), `RcDevice` instances are not [`Send`],"] # [doc = " so they can only be shared within a single thread (interrupt priority level)."] # [doc = ""] # [doc = " When this `RcDevice` is dropped, the reference count of the I2C bus will be decremented."] # [doc = " Once that reference count hits zero, it will be cleaned up."] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] pub struct RcDevice < Bus > { bus : Rc < RefCell < Bus > > , }
};
}
