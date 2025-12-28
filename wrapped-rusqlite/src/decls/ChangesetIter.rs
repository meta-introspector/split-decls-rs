macro_rules! deps {
    () => {
        ChangesetItem!();
        Changeset!();
    };
}

macro_rules! ChangesetIter {
    () => {
        deps!();
        # [doc = " Cursor for iterating over the elements of a changeset"] # [doc = " or patchset."] pub struct ChangesetIter < 'changeset > { phantom : PhantomData < & 'changeset Changeset > , it : * mut ffi :: sqlite3_changeset_iter , item : Option < ChangesetItem > , }
    };
}

ChangesetIter!();