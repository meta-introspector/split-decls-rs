macro_rules! deps {
    () => {
        OutlivesConstraint!();
    };
}

macro_rules! Trace {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Debug)] enum Trace < 'a , 'tcx > { StartRegion , FromGraph (& 'a OutlivesConstraint < 'tcx >) , FromStatic (RegionVid) , NotVisited , }
    };
}

Trace!();