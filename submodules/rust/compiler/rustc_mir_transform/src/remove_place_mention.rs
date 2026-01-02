mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use tracing :: trace ;}
mkitem!{mkstruct!{pub (super) struct RemovePlaceMention ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for RemovePlaceMention { fn is_enabled (& self , sess : & rustc_session :: Session) -> bool { ! sess . opts . unstable_opts . mir_preserve_ub } fn run_pass (& self , _ : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { trace ! ("Running RemovePlaceMention on {:?}" , body . source) ; for data in body . basic_blocks . as_mut_preserves_cfg () { data . statements . retain (| statement | match statement . kind { StatementKind :: PlaceMention (..) | StatementKind :: Nop => false , _ => true , }) } } fn is_required (& self) -> bool { true } }}}