macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! Graph {
    () => {
        deps!();
        # [doc = " A complete commit graph."] # [doc = ""] # [doc = " The data in the commit graph may come from a monolithic `objects/info/commit-graph` file, or it"] # [doc = " may come from one or more `objects/info/commit-graphs/graph-*.graph` files. These files are"] # [doc = " generated via `git commit-graph write ...` commands."] pub struct Graph { files : Vec < File > , }
    };
}

Graph!();