macro_rules! deps {
    () => {
        U32!();
        LcStr!();
        Endian!();
    };
}

macro_rules! SubUmbrellaCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SubUmbrellaCommand < E : Endian > { # [doc = " LC_SUB_UMBRELLA"] pub cmd : U32 < E > , # [doc = " includes sub_umbrella string"] pub cmdsize : U32 < E > , # [doc = " the sub_umbrella framework name"] pub sub_umbrella : LcStr < E > , }
    };
}

SubUmbrellaCommand!();