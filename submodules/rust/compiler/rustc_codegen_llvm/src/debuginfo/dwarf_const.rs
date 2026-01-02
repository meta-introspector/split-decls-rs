mkuse!{use libc :: c_uint ;}
mkitem!{# [doc = " Helper macro to let us redeclare gimli's constants as our own constants"] # [doc = " with a different type, with less risk of copy-paste errors."] macro_rules ! declare_constant { ($ name : ident : $ type : ty) => { # [allow (non_upper_case_globals)] pub (crate) const $ name : $ type = :: gimli :: constants ::$ name . 0 as $ type ; const _ : () = assert ! ($ name as i128 == :: gimli :: constants ::$ name . 0 as i128) ; } ; }}
mkitem!{declare_constant ! (DW_TAG_const_type : c_uint) ;}
mkitem!{declare_constant ! (DW_LANG_Rust : c_uint) ;}
mkitem!{declare_constant ! (DW_ATE_boolean : c_uint) ;}
mkitem!{declare_constant ! (DW_ATE_float : c_uint) ;}
mkitem!{declare_constant ! (DW_ATE_signed : c_uint) ;}
mkitem!{declare_constant ! (DW_ATE_unsigned : c_uint) ;}
mkitem!{declare_constant ! (DW_ATE_UTF : c_uint) ;}
mkitem!{declare_constant ! (DW_OP_deref : u64) ;}
mkitem!{declare_constant ! (DW_OP_plus_uconst : u64) ;}
mkitem!{# [doc = " Defined by LLVM in `llvm/include/llvm/BinaryFormat/Dwarf.h`."] # [doc = " Double-checked by a static assertion in `RustWrapper.cpp`."] # [allow (non_upper_case_globals)] pub (crate) const DW_OP_LLVM_fragment : u64 = 0x1000 ;}