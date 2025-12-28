macro_rules! deps {
    () => {
        PlaceAncestryRelation!();
    };
}

macro_rules! determine_place_ancestry_relation {
    () => {
        deps!();
        # [doc = " Determines the Ancestry relationship of Place A relative to Place B"] # [doc = ""] # [doc = " `PlaceAncestryRelation::Ancestor` implies Place A is ancestor of Place B"] # [doc = " `PlaceAncestryRelation::Descendant` implies Place A is descendant of Place B"] # [doc = " `PlaceAncestryRelation::Divergent` implies neither of them is the ancestor of the other."] fn determine_place_ancestry_relation < 'tcx > (place_a : & Place < 'tcx > , place_b : & Place < 'tcx > ,) -> PlaceAncestryRelation { if place_a . base != place_b . base { return PlaceAncestryRelation :: Divergent ; } let projections_a = & place_a . projections ; let projections_b = & place_b . projections ; let same_initial_projections = iter :: zip (projections_a , projections_b) . all (| (proj_a , proj_b) | proj_a . kind == proj_b . kind) ; if same_initial_projections { use std :: cmp :: Ordering ; match projections_b . len () . cmp (& projections_a . len ()) { Ordering :: Greater => PlaceAncestryRelation :: Ancestor , Ordering :: Equal => PlaceAncestryRelation :: SamePlace , Ordering :: Less => PlaceAncestryRelation :: Descendant , } } else { PlaceAncestryRelation :: Divergent } }
    };
}

determine_place_ancestry_relation!();