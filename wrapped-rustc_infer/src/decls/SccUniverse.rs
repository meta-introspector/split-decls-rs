macro_rules! SccUniverse {
    () => {
        # [doc = " Tracks the \"minimum universe\" for each SCC, along with some region that"] # [doc = " caused it to change."] # [derive (Copy , Clone , Debug)] struct SccUniverse < 'tcx > { # [doc = " For some SCC S, the minimum universe of:"] # [doc = ""] # [doc = " * each region R in S"] # [doc = " * each SCC S1 such that S: S1"] universe : ty :: UniverseIndex , # [doc = " Some region that caused `universe` to be what it is."] region : Option < ty :: Region < 'tcx > > , }
    };
}

SccUniverse!()