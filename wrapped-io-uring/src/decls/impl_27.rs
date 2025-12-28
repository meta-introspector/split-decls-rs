macro_rules! deps {
    () => {
        Entry32!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Entry32 { # [doc = " The operation-specific result code. For example, for a [`Read`](crate::opcode::Read)"] # [doc = " operation this is equivalent to the return value of the `read(2)` system call."] # [inline] pub fn result (& self) -> i32 { self . 0 . 0 . res } # [doc = " The user data of the request, as set by"] # [doc = " [`Entry::user_data`](crate::squeue::Entry::user_data) on the submission queue event."] # [inline] pub fn user_data (& self) -> u64 { self . 0 . 0 . user_data } # [doc = " Metadata related to the operation."] # [doc = ""] # [doc = " This is currently used for:"] # [doc = " - Storing the selected buffer ID, if one was selected. See"] # [doc = "   [`BUFFER_SELECT`](crate::squeue::Flags::BUFFER_SELECT) for more info."] # [inline] pub fn flags (& self) -> u32 { self . 0 . 0 . flags } # [doc = " Additional data available in 32-byte completion queue entries (CQEs)."] # [inline] pub fn big_cqe (& self) -> & [u64 ; 2] { & self . 1 } }
    };
}

impl_27!();