mkuse!{use std :: fs :: File ;}
mkuse!{use std :: io ;}
mkuse!{use rustc_middle :: mir :: { Body , write_mir_pretty } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: config :: { OutFileName , OutputType } ;}
mkitem!{mkstruct!{pub (super) struct Marker (pub & 'static str) ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for Marker { fn name (& self) -> & 'static str { self . 0 } fn run_pass (& self , _tcx : TyCtxt < 'tcx > , _body : & mut Body < 'tcx >) { } fn is_required (& self) -> bool { false } }}}

macro_rules! emit_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_mir in module {}", module_path!());
    };
}

mkfn!{
    emit_mir_introspect!();
    pub fn emit_mir (tcx : TyCtxt < '_ >) -> io :: Result < () > { match tcx . output_filenames (()) . path (OutputType :: Mir) { OutFileName :: Stdout => { let mut f = io :: stdout () ; write_mir_pretty (tcx , None , & mut f) ? ; } OutFileName :: Real (path) => { let mut f = File :: create_buffered (& path) ? ; write_mir_pretty (tcx , None , & mut f) ? ; if tcx . sess . opts . json_artifact_notifications { tcx . dcx () . emit_artifact_notification (& path , "mir") ; } } } Ok (()) }
}