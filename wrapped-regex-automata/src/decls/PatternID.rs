macro_rules! deps {
    () => {
        SmallIndex!();
    };
}

macro_rules! PatternID {
    () => {
        deps!();
        # [doc = " The identifier of a regex pattern, represented by a [`SmallIndex`]."] # [doc = ""] # [doc = " The identifier for a pattern corresponds to its relative position among"] # [doc = " other patterns in a single finite state machine. Namely, when building"] # [doc = " a multi-pattern regex engine, one must supply a sequence of patterns to"] # [doc = " match. The position (starting at 0) of each pattern in that sequence"] # [doc = " represents its identifier. This identifier is in turn used to identify and"] # [doc = " report matches of that pattern in various APIs."] # [doc = ""] # [doc = " See the [`SmallIndex`] type for more information about what it means for"] # [doc = " a pattern ID to be a \"small index.\""] # [doc = ""] # [doc = " Note that this type is defined in the"] # [doc = " [`util::primitives`](crate::util::primitives) module, but it is also"] # [doc = " re-exported at the crate root due to how common it is."] # [derive (Clone , Copy , Default , Eq , Hash , PartialEq , PartialOrd , Ord)] # [repr (transparent)] pub struct PatternID (SmallIndex) ;
    };
}

PatternID!();