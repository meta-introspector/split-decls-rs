mkitem!{mkstruct!{# [doc = " An owned immutable value."] # [derive (Debug , Clone)] pub struct Frozen < T > (T) ;}}
mkitem!{mkimpl!{impl < T > Frozen < T > { pub fn freeze (val : T) -> Self { Frozen (val) } }}}
mkitem!{mkimpl!{impl < T > std :: ops :: Deref for Frozen < T > { type Target = T ; fn deref (& self) -> & T { & self . 0 } }}}