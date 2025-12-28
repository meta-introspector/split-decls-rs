macro_rules! deps {
    () => {
        Merge!();
    };
}

macro_rules! InProgress {
    () => {
        deps!();
        # [doc = " Tell what operation is currently in progress."] # [derive (Debug , PartialEq , Eq)] pub enum InProgress { # [doc = " A mailbox is being applied."] ApplyMailbox , # [doc = " A rebase is happening while a mailbox is being applied."] ApplyMailboxRebase , # [doc = " A git bisect operation has not yet been concluded."] Bisect , # [doc = " A cherry pick operation."] CherryPick , # [doc = " A cherry pick with multiple commits pending."] CherryPickSequence , # [doc = " A merge operation."] Merge , # [doc = " A rebase operation."] Rebase , # [doc = " An interactive rebase operation."] RebaseInteractive , # [doc = " A revert operation."] Revert , # [doc = " A revert operation with multiple commits pending."] RevertSequence , }
    };
}

InProgress!();