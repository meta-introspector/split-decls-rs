macro_rules! deps {
    () => {
        ReadBufCursor!();
        Result!();
        Read!();
        Rewind!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < T > Read for Rewind < T > where T : Read + Unpin , { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , mut buf : ReadBufCursor < '_ > ,) -> Poll < io :: Result < () > > { if let Some (mut prefix) = self . pre . take () { if ! prefix . is_empty () { let copy_len = cmp :: min (prefix . len () , buf . remaining ()) ; buf . put_slice (& prefix [.. copy_len]) ; prefix . advance (copy_len) ; if ! prefix . is_empty () { self . pre = Some (prefix) ; } return Poll :: Ready (Ok (())) ; } } Pin :: new (& mut self . inner) . poll_read (cx , buf) } }
    };
}

impl_75!()