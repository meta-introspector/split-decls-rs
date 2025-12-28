macro_rules! deps {
    () => {
        State!();
        CommitRefIter!();
    };
}

macro_rules! CommitRefIterRaw {
    () => {
        deps!();
        # [doc = " A variation of [`CommitRefIter`] that return's [`RawToken`]s instead."] struct CommitRefIterRaw < 'a > { data : & 'a [u8] , state : State , offset : usize , }
    };
}

CommitRefIterRaw!();