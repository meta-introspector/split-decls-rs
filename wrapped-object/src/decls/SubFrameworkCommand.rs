macro_rules! deps {
    () => {
        U32!();
        Endian!();
        LcStr!();
    };
}

macro_rules! SubFrameworkCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SubFrameworkCommand < E : Endian > { # [doc = " LC_SUB_FRAMEWORK"] pub cmd : U32 < E > , # [doc = " includes umbrella string"] pub cmdsize : U32 < E > , # [doc = " the umbrella framework name"] pub umbrella : LcStr < E > , }
    };
}

SubFrameworkCommand!()