// Generated macro for RcDevice (struct)
macro_rules! Depcrate_spi_rcRcDevice {
() => {
// Module: crate::spi::rc
// Provides: {"RcDevice"}
// Dependencies: {}
# [doc = " Implementation of [`SpiDevice`] around a bus shared with `Rc<RefCell<T>>`."] # [doc = " This is the reference-counting equivalent of [`RefCellDevice`](super::RefCellDevice), requiring allocation."] # [doc = ""] # [doc = " A single [`SpiBus`] is shared via [`RefCell`], and its ownership is handled by [`Rc`]."] # [doc = " Both of these mechanisms only allow sharing within a single thread (or interrupt priority level)."] # [doc = " For this reason, this does not implement [`Send`]."] # [doc = ""] # [doc = " When this structure is dropped, the reference count of the `Bus` instance will be decremented,"] # [doc = " and it will be cleaned up once the reference count reaches zero."] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] pub struct RcDevice < Bus , Cs , Delay > { bus : Rc < RefCell < Bus > > , cs : Cs , delay : Delay , }
};
}
