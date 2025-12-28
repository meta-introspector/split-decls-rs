macro_rules! MaybeReversedEdges {
    () => {
        # [doc = " An edges iterator which may reverse the edge orientation."] # [derive (Debug , Clone)] pub struct MaybeReversedEdges < I > { iter : I , reversed : bool , }
    };
}

MaybeReversedEdges!();