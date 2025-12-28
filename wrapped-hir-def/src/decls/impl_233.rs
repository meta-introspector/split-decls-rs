macro_rules! deps {
    () => {
        AsmOptions!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl AsmOptions { pub const COUNT : usize = Self :: all () . bits () . count_ones () as usize ; pub const GLOBAL_OPTIONS : Self = Self :: ATT_SYNTAX . union (Self :: RAW) ; pub const NAKED_OPTIONS : Self = Self :: ATT_SYNTAX . union (Self :: RAW) . union (Self :: NORETURN) ; pub fn human_readable_names (& self) -> Vec < & 'static str > { let mut options = vec ! [] ; if self . contains (AsmOptions :: PURE) { options . push ("pure") ; } if self . contains (AsmOptions :: NOMEM) { options . push ("nomem") ; } if self . contains (AsmOptions :: READONLY) { options . push ("readonly") ; } if self . contains (AsmOptions :: PRESERVES_FLAGS) { options . push ("preserves_flags") ; } if self . contains (AsmOptions :: NORETURN) { options . push ("noreturn") ; } if self . contains (AsmOptions :: NOSTACK) { options . push ("nostack") ; } if self . contains (AsmOptions :: ATT_SYNTAX) { options . push ("att_syntax") ; } if self . contains (AsmOptions :: RAW) { options . push ("raw") ; } if self . contains (AsmOptions :: MAY_UNWIND) { options . push ("may_unwind") ; } options } }
    };
}

impl_233!()