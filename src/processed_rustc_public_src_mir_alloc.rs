/* FP:alloc.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_USE_0001
/* FP:alloc.rs-0002 */ use std :: io :: Read ;
/* FP:alloc.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_USE_0002
/* FP:alloc.rs-0004 */ use serde :: Serialize ;
/* FP:alloc.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_USE_0003
/* FP:alloc.rs-0006 */ use crate :: mir :: mono :: { Instance , StaticDef } ;
/* FP:alloc.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_USE_0004
/* FP:alloc.rs-0008 */ use crate :: target :: { Endian , MachineInfo } ;
/* FP:alloc.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_USE_0005
/* FP:alloc.rs-0010 */ use crate :: ty :: { Allocation , Binder , ExistentialTraitRef , Ty } ;
/* FP:alloc.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_USE_0006
/* FP:alloc.rs-0012 */ use crate :: { Error , IndexedVal , with } ;
/* FP:alloc.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_ENUM_0007
/* FP:alloc.rs-0014 */ # [doc = " An allocation in the rustc_public's IR global memory can be either a function pointer,"] # [doc = " a static, or a \"real\" allocation with some data in it."] # [derive (Debug , Clone , Eq , PartialEq , Serialize)] pub enum GlobalAlloc { # [doc = " The alloc ID is used as a function pointer."] Function (Instance) , # [doc = " This alloc ID points to a symbolic (not-reified) vtable."] # [doc = " The `None` trait ref is used to represent auto traits."] VTable (Ty , Option < Binder < ExistentialTraitRef > >) , # [doc = " The alloc ID points to a \"lazy\" static variable that did not get computed (yet)."] # [doc = " This is also used to break the cycle in recursive statics."] Static (StaticDef) , # [doc = " The alloc ID points to memory."] Memory (Allocation) , # [doc = " The first pointer-sized segment of a type id. On 64 bit systems, the 128 bit type id"] # [doc = " is split into two segments, on 32 bit systems there are 4 segments, and so on."] TypeId { ty : Ty } , }
/* FP:alloc.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_IMPL_0008
/* FP:alloc.rs-0016 */ impl From < AllocId > for GlobalAlloc { fn from (value : AllocId) -> Self { with (| cx | cx . global_alloc (value)) } }
/* FP:alloc.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_IMPL_0009
/* FP:alloc.rs-0018 */ impl GlobalAlloc { # [doc = " Retrieve the allocation id for a global allocation if it exists."] # [doc = ""] # [doc = " For `[GlobalAlloc::VTable]`, this will return the allocation for the VTable of the given"] # [doc = " type for the optional trait if the type implements the trait."] # [doc = ""] # [doc = " This method will always return `None` for allocations other than `[GlobalAlloc::VTable]`."] pub fn vtable_allocation (& self) -> Option < AllocId > { with (| cx | cx . vtable_allocation (self)) } }
/* FP:alloc.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_STRUCT_0010
/* FP:alloc.rs-0020 */ # [doc = " A unique identification number for each provenance"] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub struct AllocId (usize) ;
/* FP:alloc.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_IMPL_0011
/* FP:alloc.rs-0022 */ impl IndexedVal for AllocId { fn to_val (index : usize) -> Self { AllocId (index) } fn to_index (& self) -> usize { self . 0 } }
/* FP:alloc.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_FN_0012
/* FP:alloc.rs-0024 */ # [doc = " Utility function used to read an allocation data into a unassigned integer."] pub (crate) fn read_target_uint (mut bytes : & [u8]) -> Result < u128 , Error > { let mut buf = [0u8 ; size_of :: < u128 > ()] ; match MachineInfo :: target_endianness () { Endian :: Little => { bytes . read_exact (& mut buf [.. bytes . len ()]) ? ; Ok (u128 :: from_le_bytes (buf)) } Endian :: Big => { bytes . read_exact (& mut buf [16 - bytes . len () ..]) ? ; Ok (u128 :: from_be_bytes (buf)) } } }
/* FP:alloc.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_mir_alloc_FN_0013
/* FP:alloc.rs-0026 */ # [doc = " Utility function used to read an allocation data into an assigned integer."] pub (crate) fn read_target_int (mut bytes : & [u8]) -> Result < i128 , Error > { let mut buf = [0u8 ; size_of :: < i128 > ()] ; match MachineInfo :: target_endianness () { Endian :: Little => { bytes . read_exact (& mut buf [.. bytes . len ()]) ? ; Ok (i128 :: from_le_bytes (buf)) } Endian :: Big => { bytes . read_exact (& mut buf [16 - bytes . len () ..]) ? ; Ok (i128 :: from_be_bytes (buf)) } } }