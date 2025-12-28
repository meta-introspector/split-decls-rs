macro_rules! deps {
    () => {
        LintLevelSets!();
    };
}

macro_rules! TopDown {
    () => {
        deps!();
        pub struct TopDown { sets : LintLevelSets , cur : LintStackIndex , }
    };
}

TopDown!()