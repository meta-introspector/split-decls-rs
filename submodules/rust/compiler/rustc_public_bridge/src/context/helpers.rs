mkuse!{use rustc_middle :: mir :: interpret :: AllocRange ;}
mkuse!{use rustc_middle :: ty ;}
mkuse!{use rustc_middle :: ty :: Ty ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkitem!{mktrait!{pub trait TyHelpers < 'tcx > { fn new_foreign (& self , def_id : DefId) -> Ty < 'tcx > ; }}}
mkitem!{mktrait!{pub trait TypingEnvHelpers < 'tcx > { fn fully_monomorphized (& self) -> ty :: TypingEnv < 'tcx > ; }}}
mkitem!{mktrait!{pub trait AllocRangeHelpers < 'tcx > { fn alloc_range (& self , offset : rustc_abi :: Size , size : rustc_abi :: Size) -> AllocRange ; }}}