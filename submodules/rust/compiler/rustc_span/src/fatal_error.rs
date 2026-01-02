mkitem!{mkstruct!{# [doc = " Used as a return value to signify a fatal error occurred."] # [derive (Copy , Clone , Debug)] # [must_use] pub struct FatalError ;}}
mkuse!{pub use rustc_data_structures :: FatalErrorMarker ;}
mkitem!{mkimpl!{impl ! Send for FatalError { }}}
mkitem!{mkimpl!{impl FatalError { pub fn raise (self) -> ! { std :: panic :: resume_unwind (Box :: new (FatalErrorMarker)) } }}}
mkitem!{mkimpl!{impl std :: fmt :: Display for FatalError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "fatal error") } }}}
mkitem!{mkimpl!{impl std :: error :: Error for FatalError { }}}