mkuse!{use rustc_const_eval :: check_consts ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use crate :: MirLint ;}
mkitem!{mkstruct!{pub (super) struct CheckLiveDrops ;}}
mkitem!{mkimpl!{impl < 'tcx > MirLint < 'tcx > for CheckLiveDrops { fn run_lint (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { check_consts :: post_drop_elaboration :: check_live_drops (tcx , body) ; } }}}