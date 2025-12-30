// Generated macro for impl_39 (impl)
macro_rules! Depcrate_fmtimpl_39 {
() => {
// Module: crate::fmt
// Provides: {"impl_39"}
// Dependencies: {}
impl Display { pub (crate) fn expand_shorthand (& mut self) { let span = self . fmt . span () ; let fmt = self . fmt . value () ; let mut read = fmt . as_str () ; let mut out = String :: new () ; let mut args = TokenStream :: new () ; while let Some (brace) = read . find ('{') { out += & read [..= brace] ; read = & read [brace + 1 ..] ; if read . starts_with ('{') { out . push ('{') ; read = & read [1 ..] ; continue ; } let next = peek_next ! (read) ; let var = match next { '0' ..= '9' => take_int (& mut read) , 'a' ..= 'z' | 'A' ..= 'Z' | '_' => take_ident (& mut read) , _ => return , } ; let ident = Ident :: new (& var , span) ; let next = peek_next ! (read) ; let arg = if cfg ! (feature = "std") && next == '}' { quote_spanned ! (span => , # ident . __displaydoc_display ()) } else { quote_spanned ! (span => , # ident) } ; args . extend (arg) ; } out += read ; self . fmt = LitStr :: new (& out , self . fmt . span ()) ; self . args = args ; } }
};
}
