macro_rules! deps {
    () => {
        QueueKey!();
        CommitState!();
    };
}

macro_rules! CommitDateQueue {
    () => {
        deps!();
        type CommitDateQueue = gix_revwalk :: PriorityQueue < QueueKey < SecondsSinceUnixEpoch > , (ObjectId , CommitState) > ;
    };
}

CommitDateQueue!()