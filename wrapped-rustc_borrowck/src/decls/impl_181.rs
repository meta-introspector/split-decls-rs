macro_rules! deps {
    () => {
        RegionTracker!();
        SccAnnotations!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl scc :: Annotations < RegionVid > for SccAnnotations < '_ , '_ , RegionTracker > { fn new (& self , element : RegionVid) -> RegionTracker { RegionTracker :: new (element , & self . definitions [element]) } fn annotate_scc (& mut self , scc : ConstraintSccIndex , annotation : RegionTracker) { let idx = self . scc_to_annotation . push (annotation) ; assert ! (idx == scc) ; } type Ann = RegionTracker ; type SccIdx = ConstraintSccIndex ; }
    };
}

impl_181!();