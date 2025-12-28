macro_rules! deps {
    () => {
        SourceRef!();
    };
}

macro_rules! Mapping {
    () => {
        deps!();
        # [doc = " A mapping from a remote to a local refs for fetches or local to remote refs for pushes."] # [doc = ""] # [doc = " Mappings are like edges in a graph, initially without any constraints."] # [derive (Debug , Clone)] pub struct Mapping < 'a , 'b > { # [doc = " The index into the initial `items` list that matched against a spec."] pub item_index : Option < usize > , # [doc = " The name of the remote side for fetches or the local one for pushes that matched."] pub lhs : SourceRef < 'a > , # [doc = " The name of the local side for fetches or the remote one for pushes that corresponds to `lhs`, if available."] pub rhs : Option < Cow < 'b , BStr > > , # [doc = " The index of the matched ref-spec as seen from the match group."] pub spec_index : usize , }
    };
}

Mapping!()