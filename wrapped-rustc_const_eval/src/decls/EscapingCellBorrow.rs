macro_rules! EscapingCellBorrow {
    () => {
        # [derive (Debug)] # [doc = " A borrow of a type that contains an `UnsafeCell` somewhere. The borrow might escape to"] # [doc = " the final value of the constant, and thus we cannot allow this (for now). We may allow"] # [doc = " it in the future for static items."] pub (crate) struct EscapingCellBorrow ;
    };
}

EscapingCellBorrow!();