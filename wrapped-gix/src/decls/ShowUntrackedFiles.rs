macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! ShowUntrackedFiles {
    () => {
        deps!();
        # [doc = " The `status.showUntrackedFiles` key."] pub type ShowUntrackedFiles = keys :: Any < validate :: ShowUntrackedFiles > ;
    };
}

ShowUntrackedFiles!();