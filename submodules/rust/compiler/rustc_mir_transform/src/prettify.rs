mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_index :: { IndexSlice , IndexVec } ;}
mkuse!{use rustc_middle :: mir :: visit :: { MutVisitor , PlaceContext , Visitor } ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: Session ;}
mkitem!{mkstruct!{# [doc = " Rearranges the basic blocks into a *reverse post-order*."] # [doc = ""] # [doc = " Thus after this pass, all the successors of a block are later than it in the"] # [doc = " `IndexVec`, unless that successor is a back-edge (such as from a loop)."] pub (super) struct ReorderBasicBlocks ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for ReorderBasicBlocks { fn is_enabled (& self , _session : & Session) -> bool { false } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let rpo : IndexVec < BasicBlock , BasicBlock > = body . basic_blocks . reverse_postorder () . iter () . copied () . collect () ; if rpo . iter () . is_sorted () { return ; } let mut updater = BasicBlockUpdater { map : rpo . invert_bijective_mapping () , tcx } ; debug_assert_eq ! (updater . map [START_BLOCK] , START_BLOCK) ; updater . visit_body (body) ; permute (body . basic_blocks . as_mut () , & updater . map) ; } fn is_required (& self) -> bool { false } }}}
mkitem!{mkstruct!{# [doc = " Rearranges the locals into *use* order."] # [doc = ""] # [doc = " Thus after this pass, a local with a smaller [`Location`] where it was first"] # [doc = " assigned or referenced will have a smaller number."] # [doc = ""] # [doc = " (Does not reorder arguments nor the [`RETURN_PLACE`].)"] pub (super) struct ReorderLocals ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for ReorderLocals { fn is_enabled (& self , _session : & Session) -> bool { false } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let mut finder = LocalFinder { map : IndexVec :: new () , seen : DenseBitSet :: new_empty (body . local_decls . len ()) , } ; for local in (0 ..= body . arg_count) . map (Local :: from_usize) { finder . track (local) ; } for (bb , bbd) in body . basic_blocks . iter_enumerated () { finder . visit_basic_block_data (bb , bbd) ; } for local in body . local_decls . indices () { finder . track (local) ; } if finder . map . iter () . is_sorted () { return ; } let mut updater = LocalUpdater { map : finder . map . invert_bijective_mapping () , tcx } ; for local in (0 ..= body . arg_count) . map (Local :: from_usize) { debug_assert_eq ! (updater . map [local] , local) ; } updater . visit_body_preserves_cfg (body) ; permute (& mut body . local_decls , & updater . map) ; } fn is_required (& self) -> bool { false } }}}

macro_rules! permute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function permute in module {}", module_path!());
    };
}

mkfn!{
    permute_introspect!();
    fn permute < I : rustc_index :: Idx + Ord , T > (data : & mut IndexVec < I , T > , map : & IndexSlice < I , I >) { let mut enumerated : Vec < _ > = std :: mem :: take (data) . into_iter_enumerated () . collect () ; enumerated . sort_by_key (| p | map [p . 0]) ; * data = enumerated . into_iter () . map (| p | p . 1) . collect () ; }
}
mkitem!{mkstruct!{struct BasicBlockUpdater < 'tcx > { map : IndexVec < BasicBlock , BasicBlock > , tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > MutVisitor < 'tcx > for BasicBlockUpdater < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_terminator (& mut self , terminator : & mut Terminator < 'tcx > , _location : Location) { terminator . successors_mut (| succ | * succ = self . map [* succ]) ; } }}}
mkitem!{mkstruct!{struct LocalFinder { map : IndexVec < Local , Local > , seen : DenseBitSet < Local > , }}}
mkitem!{mkimpl!{impl LocalFinder { fn track (& mut self , l : Local) { if self . seen . insert (l) { self . map . push (l) ; } } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for LocalFinder { fn visit_local (& mut self , l : Local , context : PlaceContext , _location : Location) { if context . is_use () { self . track (l) ; } } }}}
mkitem!{mkstruct!{struct LocalUpdater < 'tcx > { map : IndexVec < Local , Local > , tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > MutVisitor < 'tcx > for LocalUpdater < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_local (& mut self , l : & mut Local , _ : PlaceContext , _ : Location) { * l = self . map [* l] ; } }}}