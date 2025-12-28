macro_rules! deps {
    () => {
        ParentIds!();
    };
}

macro_rules! Info {
    () => {
        deps!();
        # [doc = " Information about a commit that we obtained naturally as part of the iteration."] # [derive (Debug , Clone , PartialEq , Eq , Ord , PartialOrd , Hash)] pub struct Info { # [doc = " The id of the commit."] pub id : gix_hash :: ObjectId , # [doc = " All parent ids we have encountered. Note that these will be at most one if [`Parents::First`] is enabled."] pub parent_ids : ParentIds , # [doc = " The time at which the commit was created. It will only be `Some(_)` if the chosen traversal was"] # [doc = " taking dates into consideration."] pub commit_time : Option < gix_date :: SecondsSinceUnixEpoch > , }
    };
}

Info!();