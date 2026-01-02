mkuse!{use rustc_middle :: mir :: Body ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_mir_dataflow :: rustc_peek :: sanity_check ;}
mkitem!{mkstruct!{pub (super) struct SanityCheck ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirLint < 'tcx > for SanityCheck { fn run_lint (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { sanity_check (tcx , body) ; } }}}