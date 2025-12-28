macro_rules! deps {
    () => {
        UnitMeasure!();
    };
}

macro_rules! parallel_page_rank {
    () => {
        deps!();
        # [doc = " Parallel Page Rank algorithm."] # [doc = ""] # [doc = " See [`page_rank`]."] # [cfg (feature = "rayon")] pub fn parallel_page_rank < G , D > (graph : G , damping_factor : D , nb_iter : usize , tol : Option < D > ,) -> Vec < D > where G : NodeCount + IntoEdges + NodeIndexable + core :: marker :: Sync , D : UnitMeasure + Copy + core :: marker :: Send + core :: marker :: Sync , { let node_count = graph . node_count () ; if node_count == 0 { return vec ! [] ; } assert ! (D :: zero () <= damping_factor && damping_factor <= D :: one () , "Damping factor should be between 0 et 1.") ; let mut tolerance = D :: default_tol () ; if let Some (_tol) = tol { tolerance = _tol ; } let nb = D :: from_usize (node_count) ; let mut ranks : Vec < D > = (0 .. node_count) . into_par_iter () . map (| _ | D :: one () / nb) . collect () ; for _ in 0 .. nb_iter { let pi = (0 .. node_count) . into_par_iter () . map (| v | { ranks . iter () . enumerate () . map (| (w , r) | { let (out_deg , w_points_to_v) = out_edges_info (graph , w , v) ; if w_points_to_v { damping_factor * * r / out_deg } else if out_deg == D :: zero () { damping_factor * * r / nb } else { (D :: one () - damping_factor) * * r / nb } }) . sum :: < D > () }) . collect :: < Vec < D > > () ; let sum = pi . par_iter () . map (| score | * score) . sum :: < D > () ; let new_ranks = pi . par_iter () . map (| r | * r / sum) . collect :: < Vec < D > > () ; let squared_norm_2 = new_ranks . par_iter () . zip (& ranks) . map (| (new , old) | (* new - * old) * (* new - * old)) . sum :: < D > () ; if squared_norm_2 <= tolerance { return ranks ; } else { ranks = new_ranks ; } } ranks }
    };
}

parallel_page_rank!();