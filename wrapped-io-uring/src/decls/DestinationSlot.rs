macro_rules! deps {
    () => {
        Fixed!();
    };
}

macro_rules! DestinationSlot {
    () => {
        deps!();
        # [doc = " A destination slot for sending fixed resources"] # [doc = " (e.g. [`opcode::MsgRingSendFd`](crate::opcode::MsgRingSendFd))."] # [derive (Debug , Clone , Copy)] pub struct DestinationSlot { # [doc = " Fixed slot as indexed by the kernel (target+1)."] dest : NonZeroU32 , }
    };
}

DestinationSlot!();