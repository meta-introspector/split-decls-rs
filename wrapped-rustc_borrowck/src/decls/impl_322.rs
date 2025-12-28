macro_rules! deps {
    () => {
        OutlivesConstraint!();
        RawConstraints!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < 'a , 'this , 'tcx > dot :: GraphWalk < 'this > for RawConstraints < 'a , 'tcx > { type Node = RegionVid ; type Edge = OutlivesConstraint < 'tcx > ; fn nodes (& 'this self) -> dot :: Nodes < 'this , RegionVid > { let vids : Vec < RegionVid > = self . regioncx . definitions . indices () . collect () ; vids . into () } fn edges (& 'this self) -> dot :: Edges < 'this , OutlivesConstraint < 'tcx > > { (& self . regioncx . constraints . outlives () . raw [..]) . into () } fn source (& 'this self , edge : & OutlivesConstraint < 'tcx >) -> RegionVid { edge . sup } fn target (& 'this self , edge : & OutlivesConstraint < 'tcx >) -> RegionVid { edge . sub } }
    };
}

impl_322!();