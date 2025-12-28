macro_rules! deps {
    () => {
        Mapping!();
        MatchGroup!();
    };
}

macro_rules! match_rhs {
    () => {
        deps!();
        # [doc = ""] pub mod match_rhs { use crate :: { match_group :: Mapping , MatchGroup } ; # [doc = " The outcome of any matching operation of a [`MatchGroup`]."] # [doc = ""] # [doc = " It's used to validate and process the contained [mappings](Mapping)."] # [derive (Debug , Clone)] pub struct Outcome < 'spec , 'item > { # [doc = " The match group that produced this outcome."] pub group : MatchGroup < 'spec > , # [doc = " The mappings derived from matching [items](crate::match_group::Item)."] pub mappings : Vec < Mapping < 'spec , 'item > > , } }
    };
}

match_rhs!();