// Generated macro for Frame (trait)
macro_rules! DepcrateFrame {
() => {
// Module: crate
// Provides: {"Frame"}
// Dependencies: {}
# [doc = " A CAN2.0 Frame"] pub trait Frame : Sized { # [doc = " Creates a new frame."] # [doc = ""] # [doc = " This will return `None` if the data slice is too long."] fn new (id : impl Into < Id > , data : & [u8]) -> Option < Self > ; # [doc = " Creates a new remote frame (RTR bit set)."] # [doc = ""] # [doc = " This will return `None` if the data length code (DLC) is not valid."] fn new_remote (id : impl Into < Id > , dlc : usize) -> Option < Self > ; # [doc = " Returns true if this frame is an extended frame."] fn is_extended (& self) -> bool ; # [doc = " Returns true if this frame is a standard frame."] fn is_standard (& self) -> bool { ! self . is_extended () } # [doc = " Returns true if this frame is a remote frame."] fn is_remote_frame (& self) -> bool ; # [doc = " Returns true if this frame is a data frame."] fn is_data_frame (& self) -> bool { ! self . is_remote_frame () } # [doc = " Returns the frame identifier."] fn id (& self) -> Id ; # [doc = " Returns the data length code (DLC) which is in the range 0..8."] # [doc = ""] # [doc = " For data frames the DLC value always matches the length of the data."] # [doc = " Remote frames do not carry any data, yet the DLC can be greater than 0."] fn dlc (& self) -> usize ; # [doc = " Returns the frame data (0..8 bytes in length)."] fn data (& self) -> & [u8] ; }
};
}
