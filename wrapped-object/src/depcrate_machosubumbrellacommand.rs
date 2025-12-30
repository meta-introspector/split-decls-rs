// Generated macro for SubUmbrellaCommand (struct)
macro_rules! Depcrate_machoSubUmbrellaCommand {
() => {
// Module: crate::macho
// Provides: {"SubUmbrellaCommand"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SubUmbrellaCommand < E : Endian > { # [doc = " LC_SUB_UMBRELLA"] pub cmd : U32 < E > , # [doc = " includes sub_umbrella string"] pub cmdsize : U32 < E > , # [doc = " the sub_umbrella framework name"] pub sub_umbrella : LcStr < E > , }
};
}
