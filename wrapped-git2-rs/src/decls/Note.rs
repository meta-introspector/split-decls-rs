macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! Note {
    () => {
        deps!();
        # [doc = " A structure representing a [note][note] in git."] # [doc = ""] # [doc = " [note]: http://alblue.bandlem.com/2011/11/git-tip-of-week-git-notes.html"] pub struct Note < 'repo > { raw : * mut raw :: git_note , _marker : marker :: PhantomData < & 'repo Repository > , }
    };
}

Note!()