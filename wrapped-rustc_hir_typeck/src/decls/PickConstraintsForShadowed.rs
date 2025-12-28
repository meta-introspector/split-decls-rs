macro_rules! deps {
    () => {
        Pick!();
    };
}

macro_rules! PickConstraintsForShadowed {
    () => {
        deps!();
        # [doc = " Criteria to apply when searching for a given Pick. This is used during"] # [doc = " the search for potentially shadowed methods to ensure we don't search"] # [doc = " more candidates than strictly necessary."] # [derive (Debug)] struct PickConstraintsForShadowed { autoderefs : usize , receiver_steps : Option < usize > , def_id : DefId , }
    };
}

PickConstraintsForShadowed!()