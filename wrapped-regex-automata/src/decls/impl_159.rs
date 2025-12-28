macro_rules! deps {
    () => {
        Accels!();
        Accel!();
        AccelTy!();
        DFA!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        # [cfg (feature = "dfa-build")] impl Accels < Vec < AccelTy > > { # [doc = " Create an empty sequence of accelerators for a DFA."] pub fn empty () -> Accels < Vec < AccelTy > > { Accels { accels : vec ! [0] } } # [doc = " Add an accelerator to this sequence."] # [doc = ""] # [doc = " This adds to the accelerator to the end of the sequence and therefore"] # [doc = " should be done in correspondence with its state in the DFA."] # [doc = ""] # [doc = " This panics if this results in more accelerators than AccelTy::MAX."] pub fn add (& mut self , accel : Accel) { self . accels . extend_from_slice (& accel . as_accel_tys ()) ; let len = self . len () ; self . set_len (len + 1) ; } # [doc = " Set the number of accelerators in this sequence, which is encoded in"] # [doc = " the first 4 bytes of the underlying bytes."] fn set_len (& mut self , new_len : usize) { let new_len = AccelTy :: try_from (new_len) . unwrap () ; self . accels [0] = new_len ; } }
    };
}

impl_159!();