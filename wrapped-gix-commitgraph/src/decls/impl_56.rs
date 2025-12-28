macro_rules! deps {
    () => {
        Graph!();
        Error!();
        File!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [doc = " Instantiate a `Graph` from various sources."] impl Graph { # [doc = " Instantiate a commit graph from `path` which may be a directory containing graph files or the graph file itself."] pub fn at (path : & Path) -> Result < Self , Error > { Self :: try_from (path) } # [doc = " Instantiate a commit graph from the directory containing all of its files."] pub fn from_commit_graphs_dir (path : & Path) -> Result < Self , Error > { let commit_graphs_dir = path ; let chain_file_path = commit_graphs_dir . join ("commit-graph-chain") ; let chain_file = std :: fs :: File :: open (& chain_file_path) . map_err (| e | Error :: Io { err : e , path : chain_file_path . clone () , }) ? ; let mut files = Vec :: new () ; for line in BufReader :: new (chain_file) . lines () { let hash = line . map_err (| e | Error :: Io { err : e , path : chain_file_path . clone () , }) ? ; let graph_file_path = commit_graphs_dir . join (format ! ("graph-{hash}.graph")) ; files . push (File :: at (& graph_file_path) . map_err (| e | Error :: File { err : e , path : graph_file_path . clone () , }) ?) ; } Self :: new (files) } # [doc = " Instantiate a commit graph from a `.git/objects/info/commit-graph` or"] # [doc = " `.git/objects/info/commit-graphs/graph-*.graph` file."] pub fn from_file (path : & Path) -> Result < Self , Error > { let file = File :: at (path) . map_err (| e | Error :: File { err : e , path : path . to_owned () , }) ? ; Self :: new (vec ! [file]) } # [doc = " Instantiate a commit graph from an `.git/objects/info` directory."] pub fn from_info_dir (info_dir : & Path) -> Result < Self , Error > { Self :: from_file (& info_dir . join ("commit-graph")) . or_else (| _ | Self :: from_commit_graphs_dir (& info_dir . join ("commit-graphs"))) } # [doc = " Create a new commit graph from a list of `files`."] pub fn new (files : Vec < File >) -> Result < Self , Error > { let num_commits : u64 = files . iter () . map (| f | u64 :: from (f . num_commits ())) . sum () ; if num_commits > u64 :: from (MAX_COMMITS) { return Err (Error :: TooManyCommits (num_commits)) ; } for window in files . windows (2) { let f1 = & window [0] ; let f2 = & window [1] ; if f1 . object_hash () != f2 . object_hash () { return Err (Error :: HashVersionMismatch { path1 : f1 . path () . to_owned () , hash1 : f1 . object_hash () , path2 : f2 . path () . to_owned () , hash2 : f2 . object_hash () , }) ; } } Ok (Self { files }) } }
    };
}

impl_56!();