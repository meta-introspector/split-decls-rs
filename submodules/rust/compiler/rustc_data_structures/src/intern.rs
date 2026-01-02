mkuse!{use std :: cmp :: Ordering ;}
mkuse!{use std :: fmt :: { self , Debug } ;}
mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use std :: ops :: Deref ;}
mkuse!{use std :: ptr ;}
mkuse!{use crate :: stable_hasher :: { HashStable , StableHasher } ;}
mkmod!{private, { 
                getname!(private);
                getsrc!(private);
                getpath!(private);
                get_deps!(private);
                get_crates!(private);
                mkinclude!(private);
                mkitem!{mkstruct!{# [derive (Clone , Copy , Debug)] pub struct PrivateZst ;}} 
            }}
mkitem!{mkstruct!{# [doc = " A reference to a value that is interned, and is known to be unique."] # [doc = ""] # [doc = " Note that it is possible to have a `T` and a `Interned<T>` that are (or"] # [doc = " refer to) equal but different values. But if you have two different"] # [doc = " `Interned<T>`s, they both refer to the same value, at a single location in"] # [doc = " memory. This means that equality and hashing can be done on the value's"] # [doc = " address rather than the value's contents, which can improve performance."] # [doc = ""] # [doc = " The `PrivateZst` field means you can pattern match with `Interned(v, _)`"] # [doc = " but you can only construct a `Interned` with `new_unchecked`, and not"] # [doc = " directly."] # [rustc_pass_by_value] pub struct Interned < 'a , T > (pub & 'a T , pub private :: PrivateZst) ;}}
mkitem!{mkimpl!{impl < 'a , T > Interned < 'a , T > { # [doc = " Create a new `Interned` value. The value referred to *must* be interned"] # [doc = " and thus be unique, and it *must* remain unique in the future. This"] # [doc = " function has `_unchecked` in the name but is not `unsafe`, because if"] # [doc = " the uniqueness condition is violated condition it will cause incorrect"] # [doc = " behaviour but will not affect memory safety."] # [inline] pub const fn new_unchecked (t : & 'a T) -> Self { Interned (t , private :: PrivateZst) } }}}
mkitem!{mkimpl!{impl < 'a , T > Clone for Interned < 'a , T > { fn clone (& self) -> Self { * self } }}}
mkitem!{mkimpl!{impl < 'a , T > Copy for Interned < 'a , T > { }}}
mkitem!{mkimpl!{impl < 'a , T > Deref for Interned < 'a , T > { type Target = T ; # [inline] fn deref (& self) -> & T { self . 0 } }}}
mkitem!{mkimpl!{impl < 'a , T > PartialEq for Interned < 'a , T > { # [inline] fn eq (& self , other : & Self) -> bool { ptr :: eq (self . 0 , other . 0) } }}}
mkitem!{mkimpl!{impl < 'a , T > Eq for Interned < 'a , T > { }}}
mkitem!{mkimpl!{impl < 'a , T : PartialOrd > PartialOrd for Interned < 'a , T > { fn partial_cmp (& self , other : & Interned < 'a , T >) -> Option < Ordering > { if ptr :: eq (self . 0 , other . 0) { Some (Ordering :: Equal) } else { let res = self . 0 . partial_cmp (other . 0) ; debug_assert_ne ! (res , Some (Ordering :: Equal)) ; res } } }}}
mkitem!{mkimpl!{impl < 'a , T : Ord > Ord for Interned < 'a , T > { fn cmp (& self , other : & Interned < 'a , T >) -> Ordering { if ptr :: eq (self . 0 , other . 0) { Ordering :: Equal } else { let res = self . 0 . cmp (other . 0) ; debug_assert_ne ! (res , Ordering :: Equal) ; res } } }}}
mkitem!{mkimpl!{impl < 'a , T > Hash for Interned < 'a , T > where T : Hash , { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { ptr :: hash (self . 0 , s) } }}}
mkitem!{mkimpl!{impl < T , CTX > HashStable < CTX > for Interned < '_ , T > where T : HashStable < CTX > , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . 0 . hash_stable (hcx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T : Debug > Debug for Interned < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }}}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}