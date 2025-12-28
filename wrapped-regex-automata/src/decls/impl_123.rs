macro_rules! deps {
    () => {
        DFA!();
        Prefilter!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        # [doc = " Other routines that work for all `T`."] impl < T > DFA < T > { # [doc = " Set or unset the prefilter attached to this DFA."] # [doc = ""] # [doc = " This is useful when one has deserialized a DFA from `&[u8]`."] # [doc = " Deserialization does not currently include prefilters, so if you"] # [doc = " want prefilter acceleration, you'll need to rebuild it and attach"] # [doc = " it here."] pub fn set_prefilter (& mut self , prefilter : Option < Prefilter >) { self . pre = prefilter } }
    };
}

impl_123!()