// Generated macro for impl_62 (impl)
macro_rules! Depcrate_mock_functionimpl_62 {
() => {
// Module: crate::mock_function
// Provides: {"impl_62"}
// Dependencies: {}
impl ToTokens for StaticExpectations < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let common_methods = CommonExpectationsMethods { f : self . f } ; let argnames = & self . f . argnames ; let argty = & self . f . argty ; let (ig , tg , wc) = self . f . egenerics . split_for_impl () ; let lg = lifetimes_to_generics (& self . f . alifetimes) ; let output = & self . f . output ; let predexprs = & self . f . predexprs ; let v = & self . f . privmod_vis ; let (desc_fmt , desc_args) = self . f . desc () ; let no_match_msg = quote ! (concat ! (# desc_fmt , ": No matching expectation found") , # desc_args) ; quote ! (# common_methods impl # ig Expectations # tg # wc { # [doc = " Simulate calling the real method.  Every current expectation"] # [doc = " will be checked in FIFO order and the first one with"] # [doc = " matching arguments will be used."] # v fn call # lg (& self , # (# argnames : # argty ,) *) -> # output { use :: mockall :: { ViaDebug , ViaNothing } ; let __mockall_e = self . 0 . iter () . find (| __mockall_e | __mockall_e . matches (# (# predexprs ,) *) && (! __mockall_e . is_done () || self . 0 . len () == 1)) . unwrap_or_else (|| panic ! (# no_match_msg)) ; __mockall_e . call (# (# argnames ,) *) } }) . to_tokens (tokens) ; } }
};
}
