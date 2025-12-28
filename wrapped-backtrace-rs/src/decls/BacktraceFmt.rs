macro_rules! deps {
    () => {
        PrintFmt!();
        BytesOrWideString!();
    };
}

macro_rules! BacktraceFmt {
    () => {
        deps!();
        # [doc = " A formatter for backtraces."] # [doc = ""] # [doc = " This type can be used to print a backtrace regardless of where the backtrace"] # [doc = " itself comes from. If you have a `Backtrace` type then its `Debug`"] # [doc = " implementation already uses this printing format."] pub struct BacktraceFmt < 'a , 'b > { fmt : & 'a mut fmt :: Formatter < 'b > , frame_index : usize , format : PrintFmt , print_path : & 'a mut (dyn FnMut (& mut fmt :: Formatter < '_ > , BytesOrWideString < '_ >) -> fmt :: Result + 'b) , }
    };
}

BacktraceFmt!();