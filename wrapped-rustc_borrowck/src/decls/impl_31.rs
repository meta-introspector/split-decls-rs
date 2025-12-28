macro_rules! deps {
    () => {
        ConstraintGraphDirection!();
        ConstraintGraph!();
        Successors!();
        OutlivesConstraintSet!();
        RegionGraph!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a , 'tcx , D : ConstraintGraphDirection > RegionGraph < 'a , 'tcx , D > { # [doc = " Creates a \"dependency graph\" where each region constraint `R1:"] # [doc = " R2` is treated as an edge `R1 -> R2`. We use this graph to"] # [doc = " construct SCCs for region inference but also for error"] # [doc = " reporting."] pub (crate) fn new (set : & 'a OutlivesConstraintSet < 'tcx > , constraint_graph : & 'a ConstraintGraph < D > , static_region : RegionVid ,) -> Self { Self { set , constraint_graph , static_region } } # [doc = " Given a region `R`, iterate over all regions `R1` such that"] # [doc = " there exists a constraint `R: R1`."] pub (crate) fn outgoing_regions (& self , region_sup : RegionVid) -> Successors < 'a , 'tcx , D > { if region_sup == self . static_region && D :: is_normal () { Successors :: FromStatic (self . constraint_graph . outgoing_edges_from_static ()) } else { Successors :: FromGraph (self . constraint_graph . outgoing_edges_from_graph (region_sup , self . set) ,) } } }
    };
}

impl_31!()