macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! set_global_alignment {
    () => {
        deps!();
        fn set_global_alignment < 'll > (cx : & CodegenCx < 'll , '_ > , gv : & 'll Value , mut align : Align) { if let Some (min_global) = cx . sess () . target . min_global_align { align = Ord :: max (align , min_global) ; } llvm :: set_alignment (gv , align) ; }
    };
}

set_global_alignment!();