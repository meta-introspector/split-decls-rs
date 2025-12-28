macro_rules! MaybeReversedEdgeReference {
    () => {
        # [doc = " An edge reference which may reverse the edge orientation."] # [derive (Copy , Clone , Debug)] pub struct MaybeReversedEdgeReference < R > { inner : R , reversed : bool , }
    };
}

MaybeReversedEdgeReference!()