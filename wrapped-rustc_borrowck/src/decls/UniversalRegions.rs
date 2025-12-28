macro_rules! deps {
    () => {
        DefiningTy!();
        UniversalRegionIndices!();
        RegionClassification!();
    };
}

macro_rules! UniversalRegions {
    () => {
        deps!();
        # [derive (Debug)] # [derive (Clone)] pub (crate) struct UniversalRegions < 'tcx > { indices : UniversalRegionIndices < 'tcx > , # [doc = " The vid assigned to `'static`"] pub fr_static : RegionVid , # [doc = " A special region vid created to represent the current MIR fn"] # [doc = " body. It will outlive the entire CFG but it will not outlive"] # [doc = " any other universal regions."] pub fr_fn_body : RegionVid , # [doc = " We create region variables such that they are ordered by their"] # [doc = " `RegionClassification`. The first block are globals, then"] # [doc = " externals, then locals. So, things from:"] # [doc = " - `FIRST_GLOBAL_INDEX..first_extern_index` are global,"] # [doc = " - `first_extern_index..first_local_index` are external,"] # [doc = " - `first_local_index..num_universals` are local."] first_extern_index : usize , # [doc = " See `first_extern_index`."] first_local_index : usize , # [doc = " The total number of universal region variables instantiated."] num_universals : usize , # [doc = " The \"defining\" type for this function, with all universal"] # [doc = " regions instantiated. For a closure or coroutine, this is the"] # [doc = " closure type, but for a top-level function it's the `FnDef`."] pub defining_ty : DefiningTy < 'tcx > , # [doc = " The return type of this function, with all regions replaced by"] # [doc = " their universal `RegionVid` equivalents."] # [doc = ""] # [doc = " N.B., associated types in this type have not been normalized,"] # [doc = " as the name suggests. =)"] pub unnormalized_output_ty : Ty < 'tcx > , # [doc = " The fully liberated input types of this function, with all"] # [doc = " regions replaced by their universal `RegionVid` equivalents."] # [doc = ""] # [doc = " N.B., associated types in these types have not been normalized,"] # [doc = " as the name suggests. =)"] pub unnormalized_input_tys : & 'tcx [Ty < 'tcx >] , pub yield_ty : Option < Ty < 'tcx > > , pub resume_ty : Option < Ty < 'tcx > > , }
    };
}

UniversalRegions!();