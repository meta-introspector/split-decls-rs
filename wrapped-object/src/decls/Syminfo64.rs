macro_rules! deps {
    () => {
        Endian!();
        Sym64!();
        U16!();
    };
}

macro_rules! Syminfo64 {
    () => {
        deps!();
        # [doc = " Additional information about a `Sym64`."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Syminfo64 < E : Endian > { # [doc = " Direct bindings, symbol bound to."] pub si_boundto : U16 < E > , # [doc = " Per symbol flags."] pub si_flags : U16 < E > , }
    };
}

Syminfo64!()