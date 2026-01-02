mkuse!{use core :: borrow :: { Borrow , BorrowMut } ;}
mkuse!{# [unstable (feature = "bstr" , issue = "134915")] pub use core :: bstr :: ByteStr ;}
mkuse!{use core :: bstr :: { impl_partial_eq , impl_partial_eq_n , impl_partial_eq_ord } ;}
mkuse!{use core :: cmp :: Ordering ;}
mkuse!{use core :: ops :: { Deref , DerefMut , DerefPure , Index , IndexMut , Range , RangeFrom , RangeFull , RangeInclusive , RangeTo , RangeToInclusive , } ;}
mkuse!{use core :: str :: FromStr ;}
mkuse!{use core :: { fmt , hash } ;}
mkuse!{use crate :: borrow :: { Cow , ToOwned } ;}
mkuse!{use crate :: boxed :: Box ;}
mkuse!{# [cfg (not (no_rc))] use crate :: rc :: Rc ;}
mkuse!{use crate :: string :: String ;}
mkuse!{# [cfg (all (not (no_rc) , not (no_sync) , target_has_atomic = "ptr"))] use crate :: sync :: Arc ;}
mkuse!{use crate :: vec :: Vec ;}
mkitem!{mkstruct!{# [doc = " A wrapper for `Vec<u8>` representing a human-readable string that's conventionally, but not"] # [doc = " always, UTF-8."] # [doc = ""] # [doc = " Unlike `String`, this type permits non-UTF-8 contents, making it suitable for user input,"] # [doc = " non-native filenames (as `Path` only supports native filenames), and other applications that"] # [doc = " need to round-trip whatever data the user provides."] # [doc = ""] # [doc = " A `ByteString` owns its contents and can grow and shrink, like a `Vec` or `String`. For a"] # [doc = " borrowed byte string, see [`ByteStr`](../../std/bstr/struct.ByteStr.html)."] # [doc = ""] # [doc = " `ByteString` implements `Deref` to `&Vec<u8>`, so all methods available on `&Vec<u8>` are"] # [doc = " available on `ByteString`. Similarly, `ByteString` implements `DerefMut` to `&mut Vec<u8>`,"] # [doc = " so you can modify a `ByteString` using any method available on `&mut Vec<u8>`."] # [doc = ""] # [doc = " The `Debug` and `Display` implementations for `ByteString` are the same as those for `ByteStr`,"] # [doc = " showing invalid UTF-8 as hex escapes or the Unicode replacement character, respectively."] # [unstable (feature = "bstr" , issue = "134915")] # [repr (transparent)] # [derive (Clone)] # [doc (alias = "BString")] pub struct ByteString (pub Vec < u8 >) ;}}
mkitem!{mkimpl!{impl ByteString { # [inline] pub (crate) fn as_bytes (& self) -> & [u8] { & self . 0 } # [inline] pub (crate) fn as_bytestr (& self) -> & ByteStr { ByteStr :: new (& self . 0) } # [inline] pub (crate) fn as_mut_bytestr (& mut self) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Deref for ByteString { type Target = Vec < u8 > ; # [inline] fn deref (& self) -> & Self :: Target { & self . 0 } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl DerefMut for ByteString { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }}}
mkitem!{mkimpl!{# [unstable (feature = "deref_pure_trait" , issue = "87121")] unsafe impl DerefPure for ByteString { }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl fmt :: Debug for ByteString { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_bytestr () , f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl fmt :: Display for ByteString { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . as_bytestr () , f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl AsRef < [u8] > for ByteString { # [inline] fn as_ref (& self) -> & [u8] { & self . 0 } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl AsRef < ByteStr > for ByteString { # [inline] fn as_ref (& self) -> & ByteStr { self . as_bytestr () } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl AsMut < [u8] > for ByteString { # [inline] fn as_mut (& mut self) -> & mut [u8] { & mut self . 0 } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl AsMut < ByteStr > for ByteString { # [inline] fn as_mut (& mut self) -> & mut ByteStr { self . as_mut_bytestr () } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Borrow < [u8] > for ByteString { # [inline] fn borrow (& self) -> & [u8] { & self . 0 } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Borrow < ByteStr > for ByteString { # [inline] fn borrow (& self) -> & ByteStr { self . as_bytestr () } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl BorrowMut < [u8] > for ByteString { # [inline] fn borrow_mut (& mut self) -> & mut [u8] { & mut self . 0 } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl BorrowMut < ByteStr > for ByteString { # [inline] fn borrow_mut (& mut self) -> & mut ByteStr { self . as_mut_bytestr () } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Default for ByteString { fn default () -> Self { ByteString (Vec :: new ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl From < ByteString > for Vec < u8 > { # [inline] fn from (s : ByteString) -> Self { s . 0 } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > From < & 'a ByteStr > for ByteString { # [inline] fn from (s : & 'a ByteStr) -> Self { ByteString (s . 0 . to_vec ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > From < ByteString > for Cow < 'a , ByteStr > { # [inline] fn from (s : ByteString) -> Self { Cow :: Owned (s) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > From < & 'a ByteString > for Cow < 'a , ByteStr > { # [inline] fn from (s : & 'a ByteString) -> Self { Cow :: Borrowed (s . as_bytestr ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl FromIterator < char > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = char > > (iter : T) -> Self { ByteString (iter . into_iter () . collect :: < String > () . into_bytes ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl FromIterator < u8 > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = u8 > > (iter : T) -> Self { ByteString (iter . into_iter () . collect ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > FromIterator < & 'a str > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = & 'a str > > (iter : T) -> Self { ByteString (iter . into_iter () . collect :: < String > () . into_bytes ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > FromIterator < & 'a [u8] > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = & 'a [u8] > > (iter : T) -> Self { let mut buf = Vec :: new () ; for b in iter { buf . extend_from_slice (b) ; } ByteString (buf) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > FromIterator < & 'a ByteStr > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = & 'a ByteStr > > (iter : T) -> Self { let mut buf = Vec :: new () ; for b in iter { buf . extend_from_slice (& b . 0) ; } ByteString (buf) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl FromIterator < ByteString > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = ByteString > > (iter : T) -> Self { let mut buf = Vec :: new () ; for mut b in iter { buf . append (& mut b . 0) ; } ByteString (buf) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl FromStr for ByteString { type Err = core :: convert :: Infallible ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (ByteString (s . as_bytes () . to_vec ())) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Index < usize > for ByteString { type Output = u8 ; # [inline] fn index (& self , idx : usize) -> & u8 { & self . 0 [idx] } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeFull > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , _ : RangeFull) -> & ByteStr { self . as_bytestr () } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Index < Range < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : Range < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeInclusive < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : RangeInclusive < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeFrom < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : RangeFrom < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeTo < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : RangeTo < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeToInclusive < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : RangeToInclusive < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < usize > for ByteString { # [inline] fn index_mut (& mut self , idx : usize) -> & mut u8 { & mut self . 0 [idx] } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeFull > for ByteString { # [inline] fn index_mut (& mut self , _ : RangeFull) -> & mut ByteStr { self . as_mut_bytestr () } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < Range < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : Range < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeInclusive < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : RangeInclusive < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeFrom < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : RangeFrom < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeTo < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : RangeTo < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeToInclusive < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : RangeToInclusive < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl hash :: Hash for ByteString { # [inline] fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . 0 . hash (state) ; } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Eq for ByteString { }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl PartialEq for ByteString { # [inline] fn eq (& self , other : & ByteString) -> bool { self . 0 == other . 0 } }}}
mkitem!{macro_rules ! impl_partial_eq_ord_cow { ($ lhs : ty , $ rhs : ty) => { # [allow (unused_lifetimes)] # [unstable (feature = "bstr" , issue = "134915")] impl <'a > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { let other : & [u8] = (&** other) . as_ref () ; PartialEq :: eq (self . as_bytes () , other) } } # [allow (unused_lifetimes)] # [unstable (feature = "bstr" , issue = "134915")] impl <'a > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { let this : & [u8] = (&** self) . as_ref () ; PartialEq :: eq (this , other . as_bytes ()) } } # [allow (unused_lifetimes)] # [unstable (feature = "bstr" , issue = "134915")] impl <'a > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < Ordering > { let other : & [u8] = (&** other) . as_ref () ; PartialOrd :: partial_cmp (self . as_bytes () , other) } } # [allow (unused_lifetimes)] # [unstable (feature = "bstr" , issue = "134915")] impl <'a > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < Ordering > { let this : & [u8] = (&** self) . as_ref () ; PartialOrd :: partial_cmp (this , other . as_bytes ()) } } } ; }}
mkitem!{impl_partial_eq ! (ByteString , Vec < u8 >) ;}
mkitem!{impl_partial_eq ! (ByteString , [u8]) ;}
mkitem!{impl_partial_eq ! (ByteString , & [u8]) ;}
mkitem!{impl_partial_eq ! (ByteString , String) ;}
mkitem!{impl_partial_eq ! (ByteString , str) ;}
mkitem!{impl_partial_eq ! (ByteString , & str) ;}
mkitem!{impl_partial_eq_ord ! (ByteString , ByteStr) ;}
mkitem!{impl_partial_eq_ord ! (ByteString , & ByteStr) ;}
mkitem!{impl_partial_eq_n ! (ByteString , [u8 ; N]) ;}
mkitem!{impl_partial_eq_n ! (ByteString , & [u8 ; N]) ;}
mkitem!{impl_partial_eq_ord_cow ! (ByteString , Cow <'_ , ByteStr >) ;}
mkitem!{impl_partial_eq_ord_cow ! (ByteString , Cow <'_ , str >) ;}
mkitem!{impl_partial_eq_ord_cow ! (ByteString , Cow <'_ , [u8] >) ;}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Ord for ByteString { # [inline] fn cmp (& self , other : & ByteString) -> Ordering { Ord :: cmp (& self . 0 , & other . 0) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl PartialOrd for ByteString { # [inline] fn partial_cmp (& self , other : & ByteString) -> Option < Ordering > { PartialOrd :: partial_cmp (& self . 0 , & other . 0) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl ToOwned for ByteStr { type Owned = ByteString ; # [inline] fn to_owned (& self) -> ByteString { ByteString (self . 0 . to_vec ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl TryFrom < ByteString > for String { type Error = crate :: string :: FromUtf8Error ; # [inline] fn try_from (s : ByteString) -> Result < Self , Self :: Error > { String :: from_utf8 (s . 0) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > TryFrom < & 'a ByteString > for & 'a str { type Error = crate :: str :: Utf8Error ; # [inline] fn try_from (s : & 'a ByteString) -> Result < Self , Self :: Error > { crate :: str :: from_utf8 (s . 0 . as_slice ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl Clone for Box < ByteStr > { # [inline] fn clone (& self) -> Self { Self :: from (Box :: < [u8] > :: from (& self . 0)) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > From < & 'a ByteStr > for Cow < 'a , ByteStr > { # [inline] fn from (s : & 'a ByteStr) -> Self { Cow :: Borrowed (s) } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl From < Box < [u8] > > for Box < ByteStr > { # [inline] fn from (s : Box < [u8] >) -> Box < ByteStr > { unsafe { Box :: from_raw (Box :: into_raw (s) as _) } } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl From < Box < ByteStr > > for Box < [u8] > { # [inline] fn from (s : Box < ByteStr >) -> Box < [u8] > { unsafe { Box :: from_raw (Box :: into_raw (s) as _) } } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] # [cfg (not (no_rc))] impl From < Rc < [u8] > > for Rc < ByteStr > { # [inline] fn from (s : Rc < [u8] >) -> Rc < ByteStr > { unsafe { Rc :: from_raw (Rc :: into_raw (s) as _) } } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] # [cfg (not (no_rc))] impl From < Rc < ByteStr > > for Rc < [u8] > { # [inline] fn from (s : Rc < ByteStr >) -> Rc < [u8] > { unsafe { Rc :: from_raw (Rc :: into_raw (s) as _) } } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] # [cfg (all (not (no_rc) , not (no_sync) , target_has_atomic = "ptr"))] impl From < Arc < [u8] > > for Arc < ByteStr > { # [inline] fn from (s : Arc < [u8] >) -> Arc < ByteStr > { unsafe { Arc :: from_raw (Arc :: into_raw (s) as _) } } }}}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] # [cfg (all (not (no_rc) , not (no_sync) , target_has_atomic = "ptr"))] impl From < Arc < ByteStr > > for Arc < [u8] > { # [inline] fn from (s : Arc < ByteStr >) -> Arc < [u8] > { unsafe { Arc :: from_raw (Arc :: into_raw (s) as _) } } }}}
mkitem!{impl_partial_eq ! (ByteStr , Vec < u8 >) ;}
mkitem!{impl_partial_eq ! (ByteStr , String) ;}
mkitem!{impl_partial_eq_ord_cow ! (&'a ByteStr , Cow <'a , ByteStr >) ;}
mkitem!{impl_partial_eq_ord_cow ! (&'a ByteStr , Cow <'a , str >) ;}
mkitem!{impl_partial_eq_ord_cow ! (&'a ByteStr , Cow <'a , [u8] >) ;}
mkitem!{mkimpl!{# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > TryFrom < & 'a ByteStr > for String { type Error = core :: str :: Utf8Error ; # [inline] fn try_from (s : & 'a ByteStr) -> Result < Self , Self :: Error > { Ok (core :: str :: from_utf8 (& s . 0) ? . into ()) } }}}