macro_rules! deps {
    () => {
        RegionInferenceContext!();
    };
}

macro_rules! render_region_vid {
    () => {
        deps!();
        fn render_region_vid < 'tcx > (tcx : TyCtxt < 'tcx > , rvid : RegionVid , regioncx : & RegionInferenceContext < 'tcx > ,) -> String { let universe_str = render_universe (regioncx . region_definition (rvid) . universe) ; let external_name_str = if let Some (external_name) = regioncx . region_definition (rvid) . external_name . and_then (| e | e . get_name (tcx)) { format ! (" ({external_name})") } else { "" . to_string () } ; let extra_info = match regioncx . region_definition (rvid) . origin { NllRegionVariableOrigin :: FreeRegion => "" . to_string () , NllRegionVariableOrigin :: Placeholder (p) => match p . bound . kind { ty :: BoundRegionKind :: Named (def_id) => { format ! (" (for<{}>)" , tcx . item_name (def_id)) } ty :: BoundRegionKind :: ClosureEnv | ty :: BoundRegionKind :: Anon => " (for<'_>)" . to_string () , ty :: BoundRegionKind :: NamedAnon (_) => { bug ! ("only used for pretty printing") } } , NllRegionVariableOrigin :: Existential { name : Some (name) , .. } => format ! (" (ex<{name}>)") , NllRegionVariableOrigin :: Existential { .. } => format ! (" (ex<'_>)") , } ; format ! ("{:?}{universe_str}{external_name_str}{extra_info}" , rvid) }
    };
}

render_region_vid!();