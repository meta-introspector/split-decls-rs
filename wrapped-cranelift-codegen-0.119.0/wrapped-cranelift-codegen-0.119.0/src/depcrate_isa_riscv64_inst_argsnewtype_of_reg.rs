// Generated macro for newtype_of_reg (macro)
macro_rules! Depcrate_isa_riscv64_inst_argsnewtype_of_reg {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"newtype_of_reg"}
// Dependencies: {}
# [doc = " A macro for defining a newtype of `Reg` that enforces some invariant about"] # [doc = " the wrapped `Reg` (such as that it is of a particular register class)."] macro_rules ! newtype_of_reg { ($ newtype_reg : ident , $ newtype_writable_reg : ident , |$ check_reg : ident | $ check : expr) => { # [doc = " A newtype wrapper around `Reg`."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct $ newtype_reg (Reg) ; impl PartialEq < Reg > for $ newtype_reg { fn eq (& self , other : & Reg) -> bool { self . 0 == * other } } impl From <$ newtype_reg > for Reg { fn from (r : $ newtype_reg) -> Self { r . 0 } } impl $ newtype_reg { # [doc = " Create this newtype from the given register, or return `None` if the register"] # [doc = " is not a valid instance of this newtype."] pub fn new ($ check_reg : Reg) -> Option < Self > { if $ check { Some (Self ($ check_reg)) } else { None } } # [doc = " Get this newtype's underlying `Reg`."] pub fn to_reg (self) -> Reg { self . 0 } } impl std :: ops :: Deref for $ newtype_reg { type Target = Reg ; fn deref (& self) -> & Reg { & self . 0 } } # [doc = " Writable Reg."] pub type $ newtype_writable_reg = Writable <$ newtype_reg >; } ; }
};
}
