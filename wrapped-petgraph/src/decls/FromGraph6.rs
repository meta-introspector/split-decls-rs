macro_rules! FromGraph6 {
    () => {
        # [doc = " A graph that can be converted from graph6 format string."] pub trait FromGraph6 { fn from_graph6_string (graph6_string : String) -> Self ; }
    };
}

FromGraph6!()