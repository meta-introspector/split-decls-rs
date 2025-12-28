macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! AdditionalEntry {
    () => {
        deps!();
        # [doc = " An entry that is [added to the stream][Stream::add_entry()] by the user, verbatim, without additional worktree conversions."] # [doc = ""] # [doc = " It may overwrite previously written paths, which may or may not work for the consumer of the stream."] pub struct AdditionalEntry { # [doc = " The hash of the object, uniquely identifying it."] # [doc = " Note that it can be [`null()`][gix_hash::ObjectId::null()] as the hash is typically ignored by consumers of the stream."] pub id : gix_hash :: ObjectId , # [doc = " The kind of entry to create."] pub mode : gix_object :: tree :: EntryMode , # [doc = " The path relative to the repository at which content should be located."] pub relative_path : BString , # [doc = " Where to get the content of the entry from."] pub source : entry :: Source , }
    };
}

AdditionalEntry!();