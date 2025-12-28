macro_rules! deps {
    () => {
        NodeData!();
    };
}

macro_rules! TarjanScc {
    () => {
        deps!();
        # [doc = " A reusable state for computing the *strongly connected components* using [Tarjan's algorithm][1]."] # [doc = ""] # [doc = " [1]: https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm"] # [derive (Debug)] pub struct TarjanScc < N > { index : usize , componentcount : usize , nodes : Vec < NodeData > , stack : Vec < N > , }
    };
}

TarjanScc!();