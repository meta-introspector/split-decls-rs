// Generated macro for impl_363 (impl)
macro_rules! Depcrate_machinst_regimpl_363 {
() => {
// Module: crate::machinst::reg
// Provides: {"impl_363"}
// Dependencies: {}
impl < T > Writable < T > { # [doc = " Explicitly construct a `Writable<T>` from a `T`. As noted in"] # [doc = " the documentation for `Writable`, this is not hidden or"] # [doc = " disallowed from the outside; anyone can perform the \"cast\";"] # [doc = " but it is explicit so that we can audit the use sites."] pub fn from_reg (reg : T) -> Writable < T > { Writable { reg } } # [doc = " Get the underlying register, which can be read."] pub fn to_reg (self) -> T { self . reg } # [doc = " Get a mutable borrow of the underlying register."] pub fn reg_mut (& mut self) -> & mut T { & mut self . reg } # [doc = " Map the underlying register to another value or type."] pub fn map < U > (self , f : impl Fn (T) -> U) -> Writable < U > { Writable { reg : f (self . reg) } } }
};
}
