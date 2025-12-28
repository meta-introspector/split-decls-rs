macro_rules! deps {
    () => {
        Entry!();
        DestinationSlot!();
    };
}

macro_rules! macro_93 {
    () => {
        deps!();
        opcode ! { # [doc = " Create an endpoint for communication, equivalent to `socket(2)`."] # [doc = ""] # [doc = " If the `file_index` argument is set, the resulting socket is"] # [doc = " directly mapped to the given fixed-file slot instead of being"] # [doc = " returned as a normal file descriptor. The application must first"] # [doc = " have registered a file table, and the target slot should fit into"] # [doc = " it."] # [doc = ""] # [doc = " Available since 5.19."] pub struct Socket { domain : { i32 } , socket_type : { i32 } , protocol : { i32 } , ;; file_index : Option < types :: DestinationSlot > = None , flags : i32 = 0 , } pub const CODE = sys :: IORING_OP_SOCKET ; pub fn build (self) -> Entry { let Socket { domain , socket_type , protocol , file_index , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = domain as _ ; sqe . __bindgen_anon_1 . off = socket_type as _ ; sqe . len = protocol as _ ; sqe . __bindgen_anon_3 . rw_flags = flags as _ ; if let Some (dest) = file_index { sqe . __bindgen_anon_5 . file_index = dest . kernel_index_arg () ; } Entry (sqe) } }
    };
}

macro_93!();