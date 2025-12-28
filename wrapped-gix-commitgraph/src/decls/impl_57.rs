macro_rules! deps {
    () => {
        Graph!();
        Error!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl TryFrom < & Path > for Graph { type Error = Error ; fn try_from (path : & Path) -> Result < Self , Self :: Error > { if path . is_file () { Self :: from_file (path) } else if path . is_dir () { if path . join ("commit-graph-chain") . is_file () { Self :: from_commit_graphs_dir (path) } else { Self :: from_info_dir (path) } } else { Err (Error :: InvalidPath (path . to_owned ())) } } }
    };
}

impl_57!();