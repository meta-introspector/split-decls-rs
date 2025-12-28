macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! last_hunk {
    () => {
        deps!();
        # [doc = " Note that last-hunk could be [`first_hunk()`], so the hunk must only be used accordingly."] fn last_hunk < 'a > (front : & 'a [Hunk] , ours : & 'a [Hunk] , theirs : & 'a [Hunk] , back : & 'a [Hunk]) -> & 'a Hunk { back . last () . or (theirs . last ()) . or (ours . last ()) . or (front . last ()) . expect ("at least one hunk - we aborted if there are none anywhere") }
    };
}

last_hunk!()