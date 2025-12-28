macro_rules! deps {
    () => {
        ToArgRegionsFolder!();
        RegionCtxt!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl < 'a , 'tcx > ToArgRegionsFolder < 'a , 'tcx > { fn new (rcx : & 'a RegionCtxt < 'a , 'tcx > , arg_regions : & 'a [RegionVid] ,) -> ToArgRegionsFolder < 'a , 'tcx > { ToArgRegionsFolder { rcx , erase_unknown_regions : false , arg_regions } } fn fold_non_member_arg (& mut self , arg : GenericArg < 'tcx >) -> GenericArg < 'tcx > { let prev = self . erase_unknown_regions ; self . erase_unknown_regions = true ; let res = arg . try_fold_with (self) . unwrap () ; self . erase_unknown_regions = prev ; res } fn fold_closure_args (& mut self , def_id : DefId , args : GenericArgsRef < 'tcx > ,) -> Result < GenericArgsRef < 'tcx > , RegionVid > { let generics = self . cx () . generics_of (def_id) ; self . cx () . mk_args_from_iter (args . iter () . enumerate () . map (| (index , arg) | { if index < generics . parent_count { Ok (self . fold_non_member_arg (arg)) } else { arg . try_fold_with (self) } })) } }
    };
}

impl_346!();