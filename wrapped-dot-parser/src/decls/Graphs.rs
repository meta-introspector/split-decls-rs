macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! Graphs {
    () => {
        deps!();
        # [doc = " This is a thin wrapper over vectors of `[Graph]` in order to read files with multiple dotgraph descriptions."] pub struct Graphs < A > { # [doc = " The set of graphs."] pub graphs : Vec < Graph < A > > , }
    };
}

Graphs!()