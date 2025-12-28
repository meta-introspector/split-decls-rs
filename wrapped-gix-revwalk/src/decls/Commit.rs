macro_rules! Commit {
    () => {
        # [doc = " A commit that contains all information we can obtain through the commit-graph, which is typically enough to fuel any graph iteration."] pub struct Commit < T > { # [doc = " The parents of the commit."] pub parents : SmallVec < gix_hash :: ObjectId , 1 > , # [doc = " The time at which the commit was created."] pub commit_time : SecondsSinceUnixEpoch , # [doc = " The generation of the commit, if available."] pub generation : Option < u32 > , # [doc = " Any kind of data to associate with this commit."] pub data : T , }
    };
}

Commit!()