macro_rules! asm_target_features {
    () => {
        # [doc = " Computes the set of target features used in a function for the purposes of"] # [doc = " inline assembly."] fn asm_target_features (tcx : TyCtxt < '_ > , did : DefId) -> & FxIndexSet < Symbol > { let mut target_features = tcx . sess . unstable_target_features . clone () ; if tcx . def_kind (did) . has_codegen_attrs () { let attrs = tcx . codegen_fn_attrs (did) ; target_features . extend (attrs . target_features . iter () . map (| feature | feature . name)) ; match attrs . instruction_set { None => { } Some (InstructionSetAttr :: ArmA32) => { target_features . swap_remove (& sym :: thumb_mode) ; } Some (InstructionSetAttr :: ArmT32) => { target_features . insert (sym :: thumb_mode) ; } } } tcx . arena . alloc (target_features) }
    };
}

asm_target_features!();