mkuse!{use rustc_middle :: mir :: visit :: Visitor ;}
mkuse!{use rustc_middle :: mir :: { self , BasicBlock , Location } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use tracing :: trace ;}
mkuse!{use super :: ConstCx ;}
mkuse!{use crate :: check_consts :: check :: Checker ;}
mkuse!{use crate :: check_consts :: rustc_allow_const_fn_unstable ;}

macro_rules! checking_enabled_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function checking_enabled in module {}", module_path!());
    };
}

mkfn!{
    checking_enabled_introspect!();
    # [doc = " Returns `true` if we should use the more precise live drop checker that runs after drop"] # [doc = " elaboration."] pub fn checking_enabled (ccx : & ConstCx < '_ , '_ >) -> bool { if ccx . enforce_recursive_const_stability () { return rustc_allow_const_fn_unstable (ccx . tcx , ccx . body . source . def_id () . expect_local () , sym :: const_precise_live_drops ,) ; } ccx . tcx . features () . const_precise_live_drops () }
}

macro_rules! check_live_drops_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_live_drops in module {}", module_path!());
    };
}

mkfn!{
    check_live_drops_introspect!();
    # [doc = " Look for live drops in a const context."] # [doc = ""] # [doc = " This is separate from the rest of the const checking logic because it must run after drop"] # [doc = " elaboration."] pub fn check_live_drops < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mir :: Body < 'tcx >) { let ccx = ConstCx :: new (tcx , body) ; if ccx . const_kind . is_none () { return ; } if tcx . has_attr (body . source . def_id () , sym :: rustc_do_not_const_check) { return ; } if ! checking_enabled (& ccx) { return ; } let mut visitor = CheckLiveDrops { checker : Checker :: new (& ccx) } ; visitor . visit_body (body) ; }
}
mkitem!{mkstruct!{struct CheckLiveDrops < 'mir , 'tcx > { checker : Checker < 'mir , 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for CheckLiveDrops < '_ , 'tcx > { fn visit_basic_block_data (& mut self , bb : BasicBlock , block : & mir :: BasicBlockData < 'tcx >) { trace ! ("visit_basic_block_data: bb={:?} is_cleanup={:?}" , bb , block . is_cleanup) ; if block . is_cleanup { return ; } self . super_basic_block_data (bb , block) ; } fn visit_terminator (& mut self , terminator : & mir :: Terminator < 'tcx > , location : Location) { trace ! ("visit_terminator: terminator={:?} location={:?}" , terminator , location) ; match & terminator . kind { mir :: TerminatorKind :: Drop { place : dropped_place , .. } => { self . checker . check_drop_terminator (* dropped_place , location , terminator . source_info . span ,) ; } mir :: TerminatorKind :: UnwindTerminate (_) | mir :: TerminatorKind :: Call { .. } | mir :: TerminatorKind :: TailCall { .. } | mir :: TerminatorKind :: Assert { .. } | mir :: TerminatorKind :: FalseEdge { .. } | mir :: TerminatorKind :: FalseUnwind { .. } | mir :: TerminatorKind :: CoroutineDrop | mir :: TerminatorKind :: Goto { .. } | mir :: TerminatorKind :: InlineAsm { .. } | mir :: TerminatorKind :: UnwindResume | mir :: TerminatorKind :: Return | mir :: TerminatorKind :: SwitchInt { .. } | mir :: TerminatorKind :: Unreachable | mir :: TerminatorKind :: Yield { .. } => { } } } }}}