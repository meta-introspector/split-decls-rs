macro_rules! need_bitcode_in_object {
    () => {
        fn need_bitcode_in_object (tcx : TyCtxt < '_ >) -> bool { let sess = tcx . sess ; sess . opts . cg . embed_bitcode && tcx . crate_types () . contains (& CrateType :: Rlib) && sess . opts . output_types . contains_key (& OutputType :: Exe) }
    };
}

need_bitcode_in_object!()