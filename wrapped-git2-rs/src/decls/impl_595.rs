macro_rules! deps {
    () => {
        RebaseOperation!();
        Binding!();
        RebaseOperationType!();
        Oid!();
    };
}

macro_rules! impl_595 {
    () => {
        deps!();
        impl < 'rebase > RebaseOperation < 'rebase > { # [doc = " The type of rebase operation"] pub fn kind (& self) -> Option < RebaseOperationType > { unsafe { RebaseOperationType :: from_raw ((* self . raw) . kind) } } # [doc = " The commit ID being cherry-picked. This will be populated for all"] # [doc = " operations except those of type `GIT_REBASE_OPERATION_EXEC`."] pub fn id (& self) -> Oid { unsafe { Binding :: from_raw (& (* self . raw) . id as * const _) } } # [doc = "The executable the user has requested be run.  This will only"] # [doc = " be populated for operations of type RebaseOperationType::Exec"] pub fn exec (& self) -> Option < & str > { unsafe { str :: from_utf8 (crate :: opt_bytes (self , (* self . raw) . exec) . unwrap ()) . ok () } } }
    };
}

impl_595!();