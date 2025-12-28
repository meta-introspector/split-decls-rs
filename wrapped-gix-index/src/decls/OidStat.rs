macro_rules! deps {
    () => {
        Stat!();
    };
}

macro_rules! OidStat {
    () => {
        deps!();
        # [doc = " A structure to track filesystem stat information along with an object id, linking a worktree file with what's in our ODB."] # [derive (Clone)] pub struct OidStat { # [doc = " The file system stat information"] pub stat : entry :: Stat , # [doc = " The id of the file in our ODB."] pub id : ObjectId , }
    };
}

OidStat!()