macro_rules! Snapshot {
    () => {
        pub struct Snapshot < 'tcx > { pub (crate) undo_len : usize , _marker : PhantomData < & 'tcx () > , }
    };
}

Snapshot!()