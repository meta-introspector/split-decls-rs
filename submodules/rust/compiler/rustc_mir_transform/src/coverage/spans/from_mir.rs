mkuse!{use std :: iter ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: coverage :: CoverageKind ;}
mkuse!{use rustc_middle :: mir :: { self , FakeReadCause , Statement , StatementKind , Terminator , TerminatorKind , } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use crate :: coverage :: graph :: { BasicCoverageBlock , CoverageGraph } ;}
mkitem!{mkstruct!{# [derive (Debug)] pub (crate) struct RawSpanFromMir { # [doc = " A span that has been extracted from a MIR statement/terminator, but"] # [doc = " hasn't been \"unexpanded\", so it might not lie within the function body"] # [doc = " span and might be part of an expansion with a different context."] pub (crate) raw_span : Span , pub (crate) bcb : BasicCoverageBlock , }}}

macro_rules! extract_raw_spans_from_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_raw_spans_from_mir in module {}", module_path!());
    };
}

mkfn!{
    extract_raw_spans_from_mir_introspect!();
    # [doc = " Generates an initial set of coverage spans from the statements and"] # [doc = " terminators in the function's MIR body, each associated with its"] # [doc = " corresponding node in the coverage graph."] # [doc = ""] # [doc = " This is necessarily an inexact process, because MIR isn't designed to"] # [doc = " capture source spans at the level of detail we would want for coverage,"] # [doc = " but it's good enough to be better than nothing."] pub (crate) fn extract_raw_spans_from_mir < 'tcx > (mir_body : & mir :: Body < 'tcx > , graph : & CoverageGraph ,) -> Vec < RawSpanFromMir > { let mut raw_spans = vec ! [] ; for (bcb , bcb_data) in graph . iter_enumerated () { let make_raw_span = | raw_span : Span | RawSpanFromMir { raw_span , bcb } ; for & bb in & bcb_data . basic_blocks { let bb_data = & mir_body [bb] ; let statements = bb_data . statements . iter () ; raw_spans . extend (statements . filter_map (filtered_statement_span) . map (make_raw_span)) ; let terminator = iter :: once (bb_data . terminator ()) ; raw_spans . extend (terminator . filter_map (filtered_terminator_span) . map (make_raw_span)) ; } } raw_spans }
}

macro_rules! filtered_statement_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function filtered_statement_span in module {}", module_path!());
    };
}

mkfn!{
    filtered_statement_span_introspect!();
    # [doc = " If the MIR `Statement` has a span contributive to computing coverage spans,"] # [doc = " return it; otherwise return `None`."] fn filtered_statement_span (statement : & Statement < '_ >) -> Option < Span > { match statement . kind { StatementKind :: StorageLive (_) | StatementKind :: StorageDead (_) | StatementKind :: ConstEvalCounter | StatementKind :: BackwardIncompatibleDropHint { .. } | StatementKind :: Nop => None , StatementKind :: FakeRead (box (FakeReadCause :: ForGuardBinding , _)) => None , StatementKind :: FakeRead (_) | StatementKind :: Intrinsic (..) | StatementKind :: Coverage (CoverageKind :: SpanMarker ,) | StatementKind :: Assign (_) | StatementKind :: SetDiscriminant { .. } | StatementKind :: Deinit (..) | StatementKind :: Retag (_ , _) | StatementKind :: PlaceMention (..) | StatementKind :: AscribeUserType (_ , _) => Some (statement . source_info . span) , StatementKind :: Coverage (CoverageKind :: BlockMarker { .. }) => None , StatementKind :: Coverage (CoverageKind :: VirtualCounter { .. }) => bug ! ("Unexpected coverage statement found during coverage instrumentation: {statement:?}") , } }
}

macro_rules! filtered_terminator_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function filtered_terminator_span in module {}", module_path!());
    };
}

mkfn!{
    filtered_terminator_span_introspect!();
    # [doc = " If the MIR `Terminator` has a span contributive to computing coverage spans,"] # [doc = " return it; otherwise return `None`."] fn filtered_terminator_span (terminator : & Terminator < '_ >) -> Option < Span > { match terminator . kind { TerminatorKind :: Unreachable | TerminatorKind :: Assert { .. } | TerminatorKind :: Drop { .. } | TerminatorKind :: SwitchInt { .. } | TerminatorKind :: FalseEdge { .. } | TerminatorKind :: Goto { .. } => None , TerminatorKind :: Call { ref func , .. } | TerminatorKind :: TailCall { ref func , .. } => { let mut span = terminator . source_info . span ; if let mir :: Operand :: Constant (constant) = func && span . contains (constant . span) { span = constant . span ; } Some (span) } TerminatorKind :: UnwindResume | TerminatorKind :: UnwindTerminate (_) | TerminatorKind :: Return | TerminatorKind :: Yield { .. } | TerminatorKind :: CoroutineDrop | TerminatorKind :: FalseUnwind { .. } | TerminatorKind :: InlineAsm { .. } => Some (terminator . source_info . span) , } }
}
mkitem!{mkstruct!{# [derive (Debug)] pub (crate) struct Hole { pub (crate) span : Span , }}}
mkitem!{mkimpl!{impl Hole { pub (crate) fn merge_if_overlapping_or_adjacent (& mut self , other : & mut Self) -> bool { if ! self . span . overlaps_or_adjacent (other . span) { return false ; } self . span = self . span . to (other . span) ; true } }}}