/* FP:target.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_target_USE_0001
/* FP:target.rs-0002 */ use serde :: Serialize ;
/* FP:target.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_target_USE_0002
/* FP:target.rs-0004 */ use crate :: compiler_interface :: with ;
/* FP:target.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_target_STRUCT_0003
/* FP:target.rs-0006 */ # [doc = " The properties of the target machine being compiled into."] # [derive (Clone , PartialEq , Eq , Serialize)] pub struct MachineInfo { pub endian : Endian , pub pointer_width : MachineSize , }
/* FP:target.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_target_IMPL_0004
/* FP:target.rs-0008 */ impl MachineInfo { pub fn target () -> MachineInfo { with (| cx | cx . target_info ()) } pub fn target_endianness () -> Endian { with (| cx | cx . target_info () . endian) } pub fn target_pointer_width () -> MachineSize { with (| cx | cx . target_info () . pointer_width) } }
/* FP:target.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_target_ENUM_0005
/* FP:target.rs-0010 */ # [derive (Copy , Clone , PartialEq , Eq , Serialize)] pub enum Endian { Little , Big , }
/* FP:target.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_target_STRUCT_0006
/* FP:target.rs-0012 */ # [doc = " Represent the size of a component."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub struct MachineSize { num_bits : usize , }
/* FP:target.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_target_IMPL_0007
/* FP:target.rs-0014 */ impl MachineSize { # [inline (always)] pub fn bytes (self) -> usize { self . num_bits / 8 } # [inline (always)] pub fn bits (self) -> usize { self . num_bits } # [inline (always)] pub fn from_bits (num_bits : usize) -> MachineSize { MachineSize { num_bits } } # [inline] pub fn unsigned_int_max (self) -> Option < u128 > { (self . num_bits <= 128) . then (| | u128 :: MAX >> (128 - self . bits ())) } }