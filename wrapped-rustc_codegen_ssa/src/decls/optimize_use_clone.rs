macro_rules! deps {
    () => {
        BuilderMethods!();
    };
}

macro_rules! optimize_use_clone {
    () => {
        deps!();
        fn optimize_use_clone < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (cx : & 'a Bx :: CodegenCx , mut mir : Body < 'tcx > ,) -> Body < 'tcx > { let tcx = cx . tcx () ; if tcx . features () . ergonomic_clones () { for bb in mir . basic_blocks . as_mut () { let mir :: TerminatorKind :: Call { args , destination , target , call_source : mir :: CallSource :: Use , .. } = & bb . terminator () . kind else { continue ; } ; assert_eq ! (args . len () , 1) ; let arg = & args [0] ; let arg_ty = arg . node . ty (& mir . local_decls , tcx) ; let ty :: Ref (_region , inner_ty , mir :: Mutability :: Not) = * arg_ty . kind () else { continue } ; if ! tcx . type_is_copy_modulo_regions (cx . typing_env () , inner_ty) { continue ; } let Some (arg_place) = arg . node . place () else { continue } ; let destination_block = target . unwrap () ; bb . statements . push (mir :: Statement :: new (bb . terminator () . source_info , mir :: StatementKind :: Assign (Box :: new ((* destination , mir :: Rvalue :: Use (mir :: Operand :: Copy (arg_place . project_deeper (& [mir :: ProjectionElem :: Deref] , tcx) ,)) ,))) ,)) ; bb . terminator_mut () . kind = mir :: TerminatorKind :: Goto { target : destination_block } ; } } mir }
    };
}

optimize_use_clone!()