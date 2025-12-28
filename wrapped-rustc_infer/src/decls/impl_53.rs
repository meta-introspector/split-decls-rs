macro_rules! deps {
    () => {
        FreeRegionMap!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'tcx > FreeRegionMap < 'tcx > { pub fn elements (& self) -> impl Iterator < Item = Region < 'tcx > > { self . relation . elements () . copied () } pub fn is_empty (& self) -> bool { self . relation . is_empty () } # [doc = " Tests whether `r_a <= r_b`."] # [doc = ""] # [doc = " Both regions must meet `is_free_or_static`."] # [doc = ""] # [doc = " Subtle: one tricky case that this code gets correct is as"] # [doc = " follows. If we know that `r_b: 'static`, then this function"] # [doc = " will return true, even though we don't know anything that"] # [doc = " directly relates `r_a` and `r_b`."] pub fn sub_free_regions (& self , tcx : TyCtxt < 'tcx > , r_a : Region < 'tcx > , r_b : Region < 'tcx > ,) -> bool { assert ! (r_a . is_free () && r_b . is_free ()) ; let re_static = tcx . lifetimes . re_static ; if self . check_relation (re_static , r_b) { true } else { self . check_relation (r_a , r_b) } } # [doc = " Check whether `r_a <= r_b` is found in the relation."] fn check_relation (& self , r_a : Region < 'tcx > , r_b : Region < 'tcx >) -> bool { r_a == r_b || self . relation . contains (r_a , r_b) } # [doc = " Computes the least-upper-bound of two free regions. In some"] # [doc = " cases, this is more conservative than necessary, in order to"] # [doc = " avoid making arbitrary choices. See"] # [doc = " `TransitiveRelation::postdom_upper_bound` for more details."] pub (crate) fn lub_param_regions (& self , tcx : TyCtxt < 'tcx > , r_a : Region < 'tcx > , r_b : Region < 'tcx > ,) -> Region < 'tcx > { debug ! ("lub_param_regions(r_a={:?}, r_b={:?})" , r_a , r_b) ; assert ! (r_a . is_param ()) ; assert ! (r_b . is_param ()) ; let result = if r_a == r_b { r_a } else { match self . relation . postdom_upper_bound (r_a , r_b) { None => tcx . lifetimes . re_static , Some (r) => r , } } ; debug ! ("lub_param_regions(r_a={:?}, r_b={:?}) = {:?}" , r_a , r_b , result) ; result } }
    };
}

impl_53!()