macro_rules! deps {
    () => {
        Flag!();
    };
}

macro_rules! IterNames {
    () => {
        deps!();
        # [doc = "\nAn iterator over flags values.\n\nThis iterator only yields flags values for contained, defined, named flags. Any remaining bits\nwon't be yielded, but can be found with the [`IterNames::remaining`] method.\n"] pub struct IterNames < B : 'static > { flags : & 'static [Flag < B >] , idx : usize , source : B , remaining : B , }
    };
}

IterNames!();