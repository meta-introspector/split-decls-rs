macro_rules! deps {
    () => {
        AsLockedWrite!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T : AsLockedWrite + ? Sized > AsLockedWrite for & mut T { type Write < 'w > = T :: Write < 'w > where Self : 'w ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { (* * self) . as_locked_write () } }
    };
}

impl_71!();