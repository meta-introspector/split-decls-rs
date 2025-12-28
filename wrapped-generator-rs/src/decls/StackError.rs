macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! StackError {
    () => {
        deps!();
        # [doc = " Error type returned by stack allocation methods."] # [derive (Debug)] pub enum StackError { # [doc = " Contains the maximum amount of memory allowed to be allocated as stack space."] ExceedsMaximumSize (usize) , # [doc = " Returned if some kind of I/O error happens during allocation."] IoError (io :: Error) , }
    };
}

StackError!();