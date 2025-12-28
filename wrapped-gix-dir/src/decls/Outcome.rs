macro_rules! deps {
    () => {
        Property!();
        Status!();
        PathspecMatch!();
        Kind!();
    };
}

macro_rules! Outcome {
    () => {
        deps!();
        # [doc = " The product of [`path()`] calls."] # [derive (Debug , Copy , Clone)] pub struct Outcome { # [doc = " The computed status of an entry. It can be seen as aggregate of things we know about an entry."] pub status : entry :: Status , # [doc = " An additional property."] pub property : Option < entry :: Property > , # [doc = " What the entry is on disk, or `None` if we aborted the classification early or an IO-error occurred"] # [doc = " when querying the disk."] # [doc = ""] # [doc = " Note that the index is used to avoid disk access provided its entries are marked uptodate"] # [doc = " (possibly by a prior call to update the status)."] pub disk_kind : Option < entry :: Kind > , # [doc = " What the entry looks like in the index, or `None` if we aborted early."] pub index_kind : Option < entry :: Kind > , # [doc = " If a pathspec matched, this is how it matched. Maybe `None` if computation didn't see the need to evaluate it."] pub pathspec_match : Option < PathspecMatch > , }
    };
}

Outcome!();