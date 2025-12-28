macro_rules! deps {
    () => {
        MemPlace!();
    };
}

macro_rules! MemPlaceMeta {
    () => {
        deps!();
        # [derive (Copy , Clone , Hash , PartialEq , Eq , Debug)] # [doc = " Information required for the sound usage of a `MemPlace`."] pub enum MemPlaceMeta < Prov : Provenance = CtfeProvenance > { # [doc = " The unsized payload (e.g. length for slices or vtable pointer for trait objects)."] Meta (Scalar < Prov >) , # [doc = " `Sized` types or unsized `extern type`"] None , }
    };
}

MemPlaceMeta!()