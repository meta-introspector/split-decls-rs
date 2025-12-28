macro_rules! deps {
    () => {
        Ok!();
        LoadDepGraph!();
    };
}

macro_rules! LoadResult {
    () => {
        deps!();
        # [derive (Debug)] # [doc = " Represents the result of an attempt to load incremental compilation data."] pub enum LoadResult < T > { # [doc = " Loading was successful."] Ok { # [allow (missing_docs)] data : T , } , # [doc = " The file either didn't exist or was produced by an incompatible compiler version."] DataOutOfDate , # [doc = " Loading the dep graph failed."] LoadDepGraph (PathBuf , std :: io :: Error) , }
    };
}

LoadResult!()