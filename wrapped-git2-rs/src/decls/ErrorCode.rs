macro_rules! deps {
    () => {
        Object!();
        Reference!();
        Patch!();
    };
}

macro_rules! ErrorCode {
    () => {
        deps!();
        # [doc = " An enumeration of possible errors that can happen when working with a git"] # [doc = " repository."] # [derive (PartialEq , Eq , Clone , Debug , Copy)] pub enum ErrorCode { # [doc = " Generic error"] GenericError , # [doc = " Requested object could not be found"] NotFound , # [doc = " Object exists preventing operation"] Exists , # [doc = " More than one object matches"] Ambiguous , # [doc = " Output buffer too short to hold data"] BufSize , # [doc = " User-generated error"] User , # [doc = " Operation not allowed on bare repository"] BareRepo , # [doc = " HEAD refers to branch with no commits"] UnbornBranch , # [doc = " Merge in progress prevented operation"] Unmerged , # [doc = " Reference was not fast-forwardable"] NotFastForward , # [doc = " Name/ref spec was not in a valid format"] InvalidSpec , # [doc = " Checkout conflicts prevented operation"] Conflict , # [doc = " Lock file prevented operation"] Locked , # [doc = " Reference value does not match expected"] Modified , # [doc = " Authentication error"] Auth , # [doc = " Server certificate is invalid"] Certificate , # [doc = " Patch/merge has already been applied"] Applied , # [doc = " The requested peel operation is not possible"] Peel , # [doc = " Unexpected EOF"] Eof , # [doc = " Invalid operation or input"] Invalid , # [doc = " Uncommitted changes in index prevented operation"] Uncommitted , # [doc = " Operation was not valid for a directory"] Directory , # [doc = " A merge conflict exists and cannot continue"] MergeConflict , # [doc = " Hashsum mismatch in object"] HashsumMismatch , # [doc = " Unsaved changes in the index would be overwritten"] IndexDirty , # [doc = " Patch application failed"] ApplyFail , # [doc = " The object is not owned by the current user"] Owner , # [doc = " Timeout"] Timeout , }
    };
}

ErrorCode!()