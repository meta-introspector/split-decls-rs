macro_rules! deps {
    () => {
        RegionGraph!();
        EdgesFromGraph!();
        ConstraintGraph!();
        ConstraintGraphDirection!();
        EdgesFromStatic!();
        OutlivesConstraintSet!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < D : ConstraintGraphDirection > ConstraintGraph < D > { # [doc = " Creates a \"dependency graph\" where each region constraint `R1:"] # [doc = " R2` is treated as an edge `R1 -> R2`. We use this graph to"] # [doc = " construct SCCs for region inference but also for error"] # [doc = " reporting."] pub (crate) fn new (direction : D , set : & OutlivesConstraintSet < '_ > , num_region_vars : usize ,) -> Self { let mut first_constraints = IndexVec :: from_elem_n (None , num_region_vars) ; let mut next_constraints = IndexVec :: from_elem (None , & set . outlives) ; for (idx , constraint) in set . outlives . iter_enumerated () . rev () { let head = & mut first_constraints [D :: start_region (constraint . sup , constraint . sub)] ; let next = & mut next_constraints [idx] ; debug_assert ! (next . is_none ()) ; * next = * head ; * head = Some (idx) ; } Self { _direction : direction , first_constraints , next_constraints } } # [doc = " Given the constraint set from which this graph was built"] # [doc = " creates a region graph so that you can iterate over *regions*"] # [doc = " and not constraints."] pub (crate) fn region_graph < 'a , 'tcx > (& 'a self , set : & 'a OutlivesConstraintSet < 'tcx > , static_region : RegionVid ,) -> RegionGraph < 'a , 'tcx , D > { RegionGraph :: new (set , self , static_region) } pub (crate) fn is_normal (& self) -> bool { D :: is_normal () } # [doc = " Given a region `R`, iterate over all constraints `R: R1`."] pub (crate) fn outgoing_edges_from_graph < 'a , 'tcx > (& 'a self , region_sup : RegionVid , constraints : & 'a OutlivesConstraintSet < 'tcx > ,) -> EdgesFromGraph < 'a , 'tcx , D > { EdgesFromGraph { graph : self , constraints , pointer : self . first_constraints [region_sup] } } # [doc = " Returns all regions (#53178)."] pub (crate) fn outgoing_edges_from_static (& self) -> EdgesFromStatic { EdgesFromStatic { next_static_idx : 0 , end_static_idx : self . first_constraints . len () } } }
    };
}

impl_25!();