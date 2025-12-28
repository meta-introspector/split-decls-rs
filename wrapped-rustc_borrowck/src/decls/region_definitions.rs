macro_rules! deps {
    () => {
        BorrowckInferCtxt!();
        RegionDefinition!();
        UniversalRegions!();
    };
}

macro_rules! region_definitions {
    () => {
        deps!();
        # [doc = " Determines if the region variable definitions contain"] # [doc = " placeholders, and compute them for later use."] pub (super) fn region_definitions < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , universal_regions : & UniversalRegions < 'tcx > ,) -> (Frozen < IndexVec < RegionVid , RegionDefinition < 'tcx > > > , bool) { let var_infos = infcx . get_region_var_infos () ; let mut definitions = IndexVec :: with_capacity (var_infos . len ()) ; let mut has_placeholders = false ; for info in var_infos . iter () { let origin = match info . origin { RegionVariableOrigin :: Nll (origin) => origin , _ => NllRegionVariableOrigin :: Existential { name : None } , } ; let definition = RegionDefinition { origin , universe : info . universe , external_name : None } ; has_placeholders |= matches ! (origin , NllRegionVariableOrigin :: Placeholder (_)) ; definitions . push (definition) ; } for (external_name , variable) in universal_regions . named_universal_regions_iter () { debug ! ("region {:?} has external name {:?}" , variable , external_name) ; definitions [variable] . external_name = Some (external_name) ; } (Frozen :: freeze (definitions) , has_placeholders) }
    };
}

region_definitions!()