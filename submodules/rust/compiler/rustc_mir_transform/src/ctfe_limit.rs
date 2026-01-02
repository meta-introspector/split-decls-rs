mkuse!{use rustc_data_structures :: graph :: dominators :: Dominators ;}
mkuse!{use rustc_middle :: mir :: { BasicBlock , BasicBlockData , Body , Statement , StatementKind , TerminatorKind , } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use tracing :: instrument ;}
mkitem!{mkstruct!{pub (super) struct CtfeLimit ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for CtfeLimit { # [instrument (skip (self , _tcx , body))] fn run_pass (& self , _tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let doms = body . basic_blocks . dominators () ; let indices : Vec < BasicBlock > = body . basic_blocks . iter_enumerated () . filter_map (| (node , node_data) | { if matches ! (node_data . terminator () . kind , TerminatorKind :: Call { .. } | TerminatorKind :: TailCall { .. }) || has_back_edge (doms , node , node_data) { Some (node) } else { None } }) . collect () ; for index in indices { insert_counter (body . basic_blocks_mut () . get_mut (index) . expect ("basic_blocks index {index} should exist") ,) ; } } fn is_required (& self) -> bool { true } }}}

macro_rules! has_back_edge_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_back_edge in module {}", module_path!());
    };
}

mkfn!{
    has_back_edge_introspect!();
    fn has_back_edge (doms : & Dominators < BasicBlock > , node : BasicBlock , node_data : & BasicBlockData < '_ > ,) -> bool { if ! doms . is_reachable (node) { return false ; } node_data . terminator () . successors () . any (| succ | doms . dominates (succ , node)) }
}

macro_rules! insert_counter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_counter in module {}", module_path!());
    };
}

mkfn!{
    insert_counter_introspect!();
    fn insert_counter (basic_block_data : & mut BasicBlockData < '_ >) { basic_block_data . statements . push (Statement :: new (basic_block_data . terminator () . source_info , StatementKind :: ConstEvalCounter ,)) ; }
}