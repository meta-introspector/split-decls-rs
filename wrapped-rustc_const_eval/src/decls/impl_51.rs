macro_rules! deps {
    () => {
        CheckLiveDrops!();
        InlineAsm!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for CheckLiveDrops < '_ , 'tcx > { fn visit_basic_block_data (& mut self , bb : BasicBlock , block : & mir :: BasicBlockData < 'tcx >) { trace ! ("visit_basic_block_data: bb={:?} is_cleanup={:?}" , bb , block . is_cleanup) ; if block . is_cleanup { return ; } self . super_basic_block_data (bb , block) ; } fn visit_terminator (& mut self , terminator : & mir :: Terminator < 'tcx > , location : Location) { trace ! ("visit_terminator: terminator={:?} location={:?}" , terminator , location) ; match & terminator . kind { mir :: TerminatorKind :: Drop { place : dropped_place , .. } => { self . checker . check_drop_terminator (* dropped_place , location , terminator . source_info . span ,) ; } mir :: TerminatorKind :: UnwindTerminate (_) | mir :: TerminatorKind :: Call { .. } | mir :: TerminatorKind :: TailCall { .. } | mir :: TerminatorKind :: Assert { .. } | mir :: TerminatorKind :: FalseEdge { .. } | mir :: TerminatorKind :: FalseUnwind { .. } | mir :: TerminatorKind :: CoroutineDrop | mir :: TerminatorKind :: Goto { .. } | mir :: TerminatorKind :: InlineAsm { .. } | mir :: TerminatorKind :: UnwindResume | mir :: TerminatorKind :: Return | mir :: TerminatorKind :: SwitchInt { .. } | mir :: TerminatorKind :: Unreachable | mir :: TerminatorKind :: Yield { .. } => { } } } }
    };
}

impl_51!()