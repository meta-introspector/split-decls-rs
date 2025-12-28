macro_rules! GitInfo {
    () => {
        # [derive (Debug , Clone)] pub struct GitInfo { pub repo_url : String , pub branch : String , pub commit_hash : String , }
    };
}

GitInfo!()