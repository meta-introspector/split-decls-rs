macro_rules! deps {
    () => {
        QualifResults!();
        HasMutInterior!();
        NeedsDrop!();
        NeedsNonConstDrop!();
    };
}

macro_rules! Qualifs {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct Qualifs < 'mir , 'tcx > { has_mut_interior : Option < QualifResults < 'mir , 'tcx , HasMutInterior > > , needs_drop : Option < QualifResults < 'mir , 'tcx , NeedsDrop > > , needs_non_const_drop : Option < QualifResults < 'mir , 'tcx , NeedsNonConstDrop > > , }
    };
}

Qualifs!()