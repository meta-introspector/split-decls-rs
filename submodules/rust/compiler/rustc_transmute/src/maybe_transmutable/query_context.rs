mkuse!{use crate :: layout ;}
mkitem!{mktrait!{# [doc = " Context necessary to answer the question \"Are these types transmutable?\"."] pub (crate) trait QueryContext { type Def : layout :: Def ; type Region : layout :: Region ; type Type : layout :: Type ; }}}
mkmod!{test, { 
                getname!(test);
                getsrc!(test);
                getpath!(test);
                get_deps!(test);
                get_crates!(test);
                mkinclude!(test);
                mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use super :: QueryContext ;}
mkitem!{mkstruct!{pub (crate) struct UltraMinimal < R = ! , T = ! > (PhantomData < (R , T) >) ;}}
mkitem!{mkimpl!{impl < R , T > Default for UltraMinimal < R , T > { fn default () -> Self { Self (PhantomData) } }}}
mkitem!{mkenum!{# [derive (Debug , Hash , Eq , PartialEq , Clone , Copy)] pub (crate) enum Def { HasSafetyInvariants , NoSafetyInvariants , }}}
mkitem!{mkimpl!{impl crate :: layout :: Def for Def { fn has_safety_invariants (& self) -> bool { self == & Self :: HasSafetyInvariants } }}}
mkitem!{mkimpl!{impl < R , T > QueryContext for UltraMinimal < R , T > where R : crate :: layout :: Region , T : crate :: layout :: Type , { type Def = Def ; type Region = R ; type Type = T ; }}} 
            }}
mkmod!{rustc, { 
                getname!(rustc);
                getsrc!(rustc);
                getpath!(rustc);
                get_deps!(rustc);
                get_crates!(rustc);
                mkinclude!(rustc);
                mkuse!{use rustc_middle :: ty :: { Region , Ty , TyCtxt } ;}
mkuse!{use super :: * ;}
mkitem!{mkimpl!{impl < 'tcx > super :: QueryContext for TyCtxt < 'tcx > { type Def = layout :: rustc :: Def < 'tcx > ; type Region = Region < 'tcx > ; type Type = Ty < 'tcx > ; }}} 
            }}