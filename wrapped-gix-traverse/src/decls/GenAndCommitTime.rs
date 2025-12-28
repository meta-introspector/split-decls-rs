macro_rules! GenAndCommitTime {
    () => {
        pub (in crate :: commit) type GenAndCommitTime = (u32 , i64) ;
    };
}

GenAndCommitTime!();