macro_rules! deps {
    () => {
        LocalAnalyzer!();
        BuilderMethods!();
        LocalKind!();
        FunctionCx!();
    };
}

macro_rules! non_ssa_locals {
    () => {
        deps!();
        pub (crate) fn non_ssa_locals < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (fx : & FunctionCx < 'a , 'tcx , Bx > , traversal_order : & [mir :: BasicBlock] ,) -> DenseBitSet < mir :: Local > { let mir = fx . mir ; let dominators = mir . basic_blocks . dominators () ; let locals = mir . local_decls . iter () . map (| decl | { let ty = fx . monomorphize (decl . ty) ; let layout = fx . cx . spanned_layout_of (ty , decl . source_info . span) ; if layout . is_zst () { LocalKind :: ZST } else { LocalKind :: Unused } }) . collect () ; let mut analyzer = LocalAnalyzer { fx , dominators , locals } ; for arg in mir . args_iter () { analyzer . define (arg , DefLocation :: Argument) ; } for bb in traversal_order . iter () . copied () { let data = & mir . basic_blocks [bb] ; analyzer . visit_basic_block_data (bb , data) ; } let mut non_ssa_locals = DenseBitSet :: new_empty (analyzer . locals . len ()) ; for (local , kind) in analyzer . locals . iter_enumerated () { if matches ! (kind , LocalKind :: Memory) { non_ssa_locals . insert (local) ; } } non_ssa_locals }
    };
}

non_ssa_locals!()