macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! Notes {
    () => {
        deps!();
        # [doc = " An iterator over all of the notes within a repository."] pub struct Notes < 'repo > { raw : * mut raw :: git_note_iterator , _marker : marker :: PhantomData < & 'repo Repository > , }
    };
}

Notes!();