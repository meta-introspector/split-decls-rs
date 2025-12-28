macro_rules! deps {
    () => {
        Revwalk!();
        Oid!();
    };
}

macro_rules! RevwalkWithHideCb {
    () => {
        deps!();
        # [doc = " A `Revwalk` with an associated \"hide callback\", see `with_hide_callback`"] pub struct RevwalkWithHideCb < 'repo , 'cb , C > where C : FnMut (Oid) -> bool , { revwalk : Revwalk < 'repo > , _marker : marker :: PhantomData < & 'cb C > , }
    };
}

RevwalkWithHideCb!();