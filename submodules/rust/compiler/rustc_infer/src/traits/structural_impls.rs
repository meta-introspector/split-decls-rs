mkuse!{use std :: fmt ;}
mkuse!{use rustc_middle :: ty ;}
mkuse!{use crate :: traits ;}
mkuse!{use crate :: traits :: project :: Normalized ;}
mkitem!{mkimpl!{impl < 'tcx , T : fmt :: Debug > fmt :: Debug for Normalized < 'tcx , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Normalized({:?}, {:?})" , self . value , self . obligations) } }}}
mkitem!{mkimpl!{impl < 'tcx , O : fmt :: Debug > fmt :: Debug for traits :: Obligation < 'tcx , O > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if ty :: tls :: with (| tcx | tcx . sess . verbose_internals ()) { write ! (f , "Obligation(predicate={:?}, cause={:?}, param_env={:?}, depth={})" , self . predicate , self . cause , self . param_env , self . recursion_depth) } else { write ! (f , "Obligation(predicate={:?}, depth={})" , self . predicate , self . recursion_depth) } } }}}
mkitem!{mkimpl!{impl < 'tcx > fmt :: Debug for traits :: MismatchedProjectionTypes < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "MismatchedProjectionTypes({:?})" , self . err) } }}}