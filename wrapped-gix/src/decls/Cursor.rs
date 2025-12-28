macro_rules! deps {
    () => {
        Repository!();
        Options!();
    };
}

macro_rules! Cursor {
    () => {
        deps!();
        # [doc = " A cursor at a specific portion of a tree to [edit](super::Editor)."] pub struct Cursor < 'a , 'repo > { inner : gix_object :: tree :: editor :: Cursor < 'a , 'repo > , validate : gix_validate :: path :: component :: Options , repo : & 'repo Repository , }
    };
}

Cursor!();