macro_rules! deps {
    () => {
        RefLog!();
    };
}

macro_rules! LogChange {
    () => {
        deps!();
        # [doc = " A change to the reflog."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub struct LogChange { # [doc = " How to treat the reference log."] pub mode : RefLog , # [doc = " If set, create a reflog even though it would otherwise not be the case as prohibited by general rules."] # [doc = " Note that ref-log writing might be prohibited in the entire repository which is when this flag has no effect either."] pub force_create_reflog : bool , # [doc = " The message to put into the reference log. It must be a single line, hence newlines are forbidden."] # [doc = " The string can be empty to indicate there should be no message at all."] pub message : BString , }
    };
}

LogChange!()