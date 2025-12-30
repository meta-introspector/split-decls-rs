// Generated macro for impl_136 (impl)
macro_rules! Depcrate_spi_rcimpl_136 {
() => {
// Module: crate::spi::rc
// Provides: {"impl_136"}
// Dependencies: {}
impl < Bus , Cs , Delay > RcDevice < Bus , Cs , Delay > { # [doc = " Creates a new [`RcDevice`]."] # [doc = ""] # [doc = " This sets the `cs` pin high, and returns an error if that fails."] # [doc = " It is recommended to have already set that pin high the moment it has been configured as an output, to avoid glitches."] # [doc = ""] # [doc = " This function does not increment the reference count:"] # [doc = " you will need to call `Rc::clone(&bus)` if you only have a `&Rc<RefCell<Bus>>`."] # [inline] pub fn new (bus : Rc < RefCell < Bus > > , mut cs : Cs , delay : Delay) -> Result < Self , Cs :: Error > where Cs : OutputPin , { cs . set_high () ? ; Ok (Self { bus , cs , delay }) } }
};
}
