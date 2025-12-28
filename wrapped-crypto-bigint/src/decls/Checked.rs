macro_rules! Checked {
    () => {
        # [doc = " Provides intentionally-checked arithmetic on `T`."] # [doc = ""] # [doc = " Internally this leverages the [`CtOption`] type from the [`subtle`] crate"] # [doc = " in order to handle overflows."] # [derive (Copy , Clone , Debug)] pub struct Checked < T > (pub CtOption < T >) ;
    };
}

Checked!()