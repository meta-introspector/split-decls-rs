macro_rules! deps {
    () => {
        OutlivesConstraint!();
        RegionInferenceContext!();
        RawConstraints!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl < 'a , 'this , 'tcx > dot :: Labeller < 'this > for RawConstraints < 'a , 'tcx > { type Node = RegionVid ; type Edge = OutlivesConstraint < 'tcx > ; fn graph_id (& 'this self) -> dot :: Id < 'this > { dot :: Id :: new ("RegionInferenceContext") . unwrap () } fn node_id (& 'this self , n : & RegionVid) -> dot :: Id < 'this > { dot :: Id :: new (format ! ("r{}" , n . index ())) . unwrap () } fn node_shape (& 'this self , _node : & RegionVid) -> Option < dot :: LabelText < 'this > > { Some (dot :: LabelText :: LabelStr (Cow :: Borrowed ("box"))) } fn node_label (& 'this self , n : & RegionVid) -> dot :: LabelText < 'this > { dot :: LabelText :: LabelStr (render_region_vid (self . tcx , * n , self . regioncx) . into ()) } fn edge_label (& 'this self , e : & OutlivesConstraint < 'tcx >) -> dot :: LabelText < 'this > { dot :: LabelText :: LabelStr (render_outlives_constraint (e) . into ()) } }
    };
}

impl_321!();