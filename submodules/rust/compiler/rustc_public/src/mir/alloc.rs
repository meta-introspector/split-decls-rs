mkuse!{use std :: io :: Read ;}
mkuse!{use serde :: Serialize ;}
mkuse!{use crate :: mir :: mono :: { Instance , StaticDef } ;}
mkuse!{use crate :: target :: { Endian , MachineInfo } ;}
mkuse!{use crate :: ty :: { Allocation , Binder , ExistentialTraitRef , Ty } ;}
mkuse!{use crate :: { Error , IndexedVal , with } ;}
mkitem!{mkenum!{# [doc = " An allocation in the rustc_public's IR global memory can be either a function pointer,"] # [doc = " a static, or a \"real\" allocation with some data in it."] # [derive (Debug , Clone , Eq , PartialEq , Serialize)] pub enum GlobalAlloc { # [doc = " The alloc ID is used as a function pointer."] Function (Instance) , # [doc = " This alloc ID points to a symbolic (not-reified) vtable."] # [doc = " The `None` trait ref is used to represent auto traits."] VTable (Ty , Option < Binder < ExistentialTraitRef > >) , # [doc = " The alloc ID points to a \"lazy\" static variable that did not get computed (yet)."] # [doc = " This is also used to break the cycle in recursive statics."] Static (StaticDef) , # [doc = " The alloc ID points to memory."] Memory (Allocation) , # [doc = " The first pointer-sized segment of a type id. On 64 bit systems, the 128 bit type id"] # [doc = " is split into two segments, on 32 bit systems there are 4 segments, and so on."] TypeId { ty : Ty } , }}}
mkitem!{mkimpl!{impl From < AllocId > for GlobalAlloc { fn from (value : AllocId) -> Self { with (| cx | cx . global_alloc (value)) } }}}
mkitem!{mkimpl!{impl GlobalAlloc { # [doc = " Retrieve the allocation id for a global allocation if it exists."] # [doc = ""] # [doc = " For `[GlobalAlloc::VTable]`, this will return the allocation for the VTable of the given"] # [doc = " type for the optional trait if the type implements the trait."] # [doc = ""] # [doc = " This method will always return `None` for allocations other than `[GlobalAlloc::VTable]`."] pub fn vtable_allocation (& self) -> Option < AllocId > { with (| cx | cx . vtable_allocation (self)) } }}}
mkitem!{mkstruct!{# [doc = " A unique identification number for each provenance"] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub struct AllocId (usize) ;}}
mkitem!{mkimpl!{impl IndexedVal for AllocId { fn to_val (index : usize) -> Self { AllocId (index) } fn to_index (& self) -> usize { self . 0 } }}}

macro_rules! read_target_uint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_target_uint in module {}", module_path!());
    };
}

mkfn!{
    read_target_uint_introspect!();
    # [doc = " Utility function used to read an allocation data into a unassigned integer."] pub (crate) fn read_target_uint (mut bytes : & [u8]) -> Result < u128 , Error > { let mut buf = [0u8 ; size_of :: < u128 > ()] ; match MachineInfo :: target_endianness () { Endian :: Little => { bytes . read_exact (& mut buf [.. bytes . len ()]) ? ; Ok (u128 :: from_le_bytes (buf)) } Endian :: Big => { bytes . read_exact (& mut buf [16 - bytes . len () ..]) ? ; Ok (u128 :: from_be_bytes (buf)) } } }
}

macro_rules! read_target_int_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_target_int in module {}", module_path!());
    };
}

mkfn!{
    read_target_int_introspect!();
    # [doc = " Utility function used to read an allocation data into an assigned integer."] pub (crate) fn read_target_int (mut bytes : & [u8]) -> Result < i128 , Error > { let mut buf = [0u8 ; size_of :: < i128 > ()] ; match MachineInfo :: target_endianness () { Endian :: Little => { bytes . read_exact (& mut buf [.. bytes . len ()]) ? ; Ok (i128 :: from_le_bytes (buf)) } Endian :: Big => { bytes . read_exact (& mut buf [16 - bytes . len () ..]) ? ; Ok (i128 :: from_be_bytes (buf)) } } }
}