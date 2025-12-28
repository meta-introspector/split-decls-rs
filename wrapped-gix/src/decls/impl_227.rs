macro_rules! deps {
    () => {
        Error!();
        Note!();
        State!();
        Platform!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl Platform < '_ , '_ > { # [doc = " Start a breadth-first, recursive traversal using `delegate`, for which a [`Recorder`](gix_traverse::tree::Recorder) can be used to get started."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " - Results are returned in sort order as per tree-sorting rules, files first, then directories, one level at a time."] # [doc = " - for obtaining the direct children of the tree, use [Tree::iter()] instead."] pub fn breadthfirst < V > (& self , delegate : & mut V) -> Result < () , gix_traverse :: tree :: breadthfirst :: Error > where V : gix_traverse :: tree :: Visit , { let root = gix_object :: TreeRefIter :: from_bytes (& self . root . data) ; let state = gix_traverse :: tree :: breadthfirst :: State :: default () ; gix_traverse :: tree :: breadthfirst (root , state , & self . root . repo . objects , delegate) } # [doc = " Start a depth-first, recursive traversal using `delegate`, for which a [`Recorder`](gix_traverse::tree::Recorder) can be used to get started."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " For obtaining the direct children of the tree, use [Tree::iter()] instead."] pub fn depthfirst < V > (& self , delegate : & mut V) -> Result < () , gix_traverse :: tree :: breadthfirst :: Error > where V : gix_traverse :: tree :: Visit , { let state = gix_traverse :: tree :: depthfirst :: State :: default () ; gix_traverse :: tree :: depthfirst (self . root . id , state , & self . root . repo . objects , delegate) } }
    };
}

impl_227!()