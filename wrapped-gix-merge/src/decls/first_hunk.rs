macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! first_hunk {
    () => {
        deps!();
        fn first_hunk < 'a > (front : & 'a [Hunk] , ours : & 'a [Hunk] , theirs : & 'a [Hunk] , back : & 'a [Hunk]) -> & 'a Hunk { front . first () . or (ours . first ()) . or (theirs . first ()) . or (back . first ()) . expect ("at least one hunk - we aborted if there are none anywhere") }
    };
}

first_hunk!()