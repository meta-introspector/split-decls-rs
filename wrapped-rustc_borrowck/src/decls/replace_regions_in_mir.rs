macro_rules! deps {
    () => {
        UniversalRegions!();
        BorrowckInferCtxt!();
    };
}

macro_rules! replace_regions_in_mir {
    () => {
        deps!();
        # [doc = " Rewrites the regions in the MIR to use NLL variables, also scraping out the set of universal"] # [doc = " regions (e.g., region parameters) declared on the function. That set will need to be given to"] # [doc = " `compute_regions`."] # [instrument (skip (infcx , body , promoted) , level = "debug")] pub (crate) fn replace_regions_in_mir < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , body : & mut Body < 'tcx > , promoted : & mut IndexSlice < Promoted , Body < 'tcx > > ,) -> UniversalRegions < 'tcx > { let def = body . source . def_id () . expect_local () ; debug ! (? def) ; let universal_regions = UniversalRegions :: new (infcx , def) ; renumber :: renumber_mir (infcx , body , promoted) ; if let Some (dumper) = MirDumper :: new (infcx . tcx , "renumber" , body) { dumper . dump_mir (body) ; } universal_regions }
    };
}

replace_regions_in_mir!();