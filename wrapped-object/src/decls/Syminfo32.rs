macro_rules! deps {
    () => {
        Endian!();
        U16!();
        Sym32!();
    };
}

macro_rules! Syminfo32 {
    () => {
        deps!();
        # [doc = " Additional information about a `Sym32`."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Syminfo32 < E : Endian > { # [doc = " Direct bindings, symbol bound to."] pub si_boundto : U16 < E > , # [doc = " Per symbol flags."] pub si_flags : U16 < E > , }
    };
}

Syminfo32!()