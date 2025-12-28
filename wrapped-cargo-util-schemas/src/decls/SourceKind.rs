macro_rules! deps {
    () => {
        GitReference!();
    };
}

macro_rules! SourceKind {
    () => {
        deps!();
        # [doc = " The possible kinds of code source."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum SourceKind { # [doc = " A git repository."] Git (GitReference) , # [doc = " A local path."] Path , # [doc = " A remote registry."] Registry , # [doc = " A sparse registry."] SparseRegistry , # [doc = " A local filesystem-based registry."] LocalRegistry , # [doc = " A directory-based registry."] Directory , }
    };
}

SourceKind!();