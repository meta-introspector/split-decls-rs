macro_rules! deps {
    () => {
        GraphemeClusterBreak!();
    };
}

macro_rules! macro_122 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Grapheme_Cluster_Break" ; short_name : "GCB" ; ident : GraphemeClusterBreak ; data_marker : crate :: provider :: PropertyEnumGraphemeClusterBreakV1 ; singleton : SINGLETON_PROPERTY_ENUM_GRAPHEME_CLUSTER_BREAK_V1 ; ule_ty : u8 ; }
    };
}

macro_122!()