macro_rules! deps {
    () => {
        PacketLineRef!();
    };
}

macro_rules! PacketLineOrWantedSize {
    () => {
        deps!();
        # [doc = " The result of [`hex_prefix()`] indicating either a special packet line or the amount of wanted bytes"] pub enum PacketLineOrWantedSize < 'a > { # [doc = " The special kind of packet line decoded from the hex prefix. It never contains actual data."] Line (PacketLineRef < 'a >) , # [doc = " The amount of bytes indicated by the hex prefix of the packet line."] Wanted (u16) , }
    };
}

PacketLineOrWantedSize!();