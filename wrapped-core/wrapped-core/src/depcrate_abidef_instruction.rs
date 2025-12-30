// Generated macro for def_instruction (macro)
macro_rules! Depcrate_abidef_instruction {
() => {
// Module: crate::abi
// Provides: {"def_instruction"}
// Dependencies: {}
macro_rules ! def_instruction { ($ (# [$ enum_attr : meta]) * pub enum $ name : ident <'a > { $ ($ (# [$ attr : meta]) * $ variant : ident $ ({ $ ($ field : ident : $ field_ty : ty $ (,) *) * }) ? : [$ num_popped : expr] => [$ num_pushed : expr] ,) * }) => { $ (# [$ enum_attr]) * pub enum $ name <'a > { $ ($ (# [$ attr]) * $ variant $ ({ $ ($ field : $ field_ty ,) * }) ? ,) * } impl $ name <'_ > { # [doc = " How many operands does this instruction pop from the stack?"] # [allow (unused_variables)] pub fn operands_len (& self) -> usize { match self { $ (Self ::$ variant $ ({ $ ($ field ,) * }) ? => $ num_popped ,) * } } # [doc = " How many results does this instruction push onto the stack?"] # [allow (unused_variables)] pub fn results_len (& self) -> usize { match self { $ (Self ::$ variant $ ({ $ ($ field ,) * }) ? => $ num_pushed ,) * } } } } ; }
};
}
