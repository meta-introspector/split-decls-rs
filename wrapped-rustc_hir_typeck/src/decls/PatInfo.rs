macro_rules! deps {
    () => {
        TopInfo!();
        DeclOrigin!();
        MutblCap!();
    };
}

macro_rules! PatInfo {
    () => {
        deps!();
        # [derive (Copy , Clone)] struct PatInfo < 'tcx > { binding_mode : ByRef , max_ref_mutbl : MutblCap , top_info : TopInfo < 'tcx > , decl_origin : Option < DeclOrigin < 'tcx > > , # [doc = " The depth of current pattern"] current_depth : u32 , }
    };
}

PatInfo!();