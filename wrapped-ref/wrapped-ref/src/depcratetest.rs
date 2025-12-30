// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: ValueRef ; pub (crate) use sval_test :: { assert_tokens , Token } ; pub (crate) fn assert_tokens_ref < 'sval > (value : impl ValueRef < 'sval > , tokens : & [sval_test :: Token < 'sval >] ,) { let mut actual = sval_test :: TokenBuf :: new () ; value . stream_ref (& mut actual) . unwrap () ; assert_eq ! (tokens , actual . as_tokens ()) ; } pub (crate) struct Ref < T > (pub (crate) T) ; impl < T : sval :: Value > sval :: Value for Ref < T > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S ,) -> sval :: Result { self . 0 . stream (stream) } } impl < 'sval , T : sval :: Value + ? Sized > ValueRef < 'sval > for Ref < & 'sval T > { fn stream_ref < S : sval :: Stream < 'sval > + ? Sized > (& self , stream : & mut S) -> sval :: Result { self . 0 . stream (stream) } } pub (crate) fn compat_case < 'sval > (v : & 'sval (impl sval :: Value + ValueRef < 'sval > + ? Sized) , tokens : & [Token < 'sval >] ,) { assert_tokens_ref (v , tokens) ; assert_tokens (v , tokens) ; } }
};
}
