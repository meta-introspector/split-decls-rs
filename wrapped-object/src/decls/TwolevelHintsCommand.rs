macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! TwolevelHintsCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct TwolevelHintsCommand < E : Endian > { # [doc = " LC_TWOLEVEL_HINTS"] pub cmd : U32 < E > , # [doc = " sizeof(struct TwolevelHintsCommand)"] pub cmdsize : U32 < E > , # [doc = " offset to the hint table"] pub offset : U32 < E > , # [doc = " number of hints in the hint table"] pub nhints : U32 < E > , }
    };
}

TwolevelHintsCommand!()