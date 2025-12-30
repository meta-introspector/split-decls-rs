// Generated macro for impl_314 (impl)
macro_rules! Depcrate_rust_typeimpl_314 {
() => {
// Module: crate::rust_type
// Provides: {"impl_314"}
// Dependencies: {}
impl < 'a , 'b > AttributeParser < 'a , 'b > { fn new (name : & 'a str , expected_name : & 'b str) -> Self { Self { _original_name : name , name : name . trim () , expected_name : expected_name . trim () , } } fn map (& mut self , f : impl Fn (& str) -> & str) { self . name = f (self . name) ; self . expected_name = f (self . expected_name) ; } fn set_constant_array (& mut self) { self . map (| s | { let (s , _) = s . split_once ('[') . expect ("array to contain [") ; s . trim () }) ; } # [doc = " Parse an incomplete array like:"] # [doc = " `id<MTLFunctionHandle>  _Nullable const  _Nonnull __unsafe_unretained[]`"] # [doc = " By removing the ending `[]`."] fn set_incomplete_array (& mut self) { self . map (| s | s . strip_suffix ("[]") . expect ("array to end with []") . trim ()) ; } # [doc = " Parse a function pointer like:"] # [doc = " `void (^ _Nonnull __strong)(...)`"] # [doc = " By extracting the inner data to:"] # [doc = " `^ _Nonnull __strong`"] fn set_fn_ptr (& mut self) { self . map (| s | { let (_ , s) = s . split_once ('(') . expect ("fn to have begin parenthesis") ; let (s , _) = s . split_once (')') . expect ("fn to have end parenthesis") ; s . trim () }) ; } fn set_inner_pointer (& mut self) { if let Some (rest) = self . name . strip_suffix ('*') { self . name = rest . trim () ; } else { error ! (? self , "expected pointer to have star") ; } } }
};
}
