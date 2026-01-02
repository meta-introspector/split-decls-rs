mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use std :: { fmt , ptr } ;}
mkuse!{use crate :: llvm ;}
mkuse!{pub (crate) use crate :: llvm :: Value ;}
mkitem!{mkimpl!{impl PartialEq for Value { fn eq (& self , other : & Self) -> bool { ptr :: eq (self , other) } }}}
mkitem!{mkimpl!{impl Eq for Value { }}}
mkitem!{mkimpl!{impl Hash for Value { fn hash < H : Hasher > (& self , hasher : & mut H) { (self as * const Self) . hash (hasher) ; } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Value { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteValueToString (self , s) ; }) . expect ("non-UTF8 value description from LLVM") ,) } }}}