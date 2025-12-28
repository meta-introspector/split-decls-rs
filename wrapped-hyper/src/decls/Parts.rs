macro_rules! deps {
    () => {
        Upgraded!();
    };
}

macro_rules! Parts {
    () => {
        deps!();
        # [doc = " The deconstructed parts of an [`Upgraded`] type."] # [doc = ""] # [doc = " Includes the original IO type, and a read buffer of bytes that the"] # [doc = " HTTP state machine may have already read before completing an upgrade."] # [derive (Debug)] # [non_exhaustive] pub struct Parts < T > { # [doc = " The original IO object used before the upgrade."] pub io : T , # [doc = " A buffer of bytes that have been read but not processed as HTTP."] # [doc = ""] # [doc = " For instance, if the `Connection` is used for an HTTP upgrade request,"] # [doc = " it is possible the server sent back the first bytes of the new protocol"] # [doc = " along with the response upgrade."] # [doc = ""] # [doc = " You will want to check for any existing bytes if you plan to continue"] # [doc = " communicating on the IO object."] pub read_buf : Bytes , }
    };
}

Parts!();