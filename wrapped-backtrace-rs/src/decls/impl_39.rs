macro_rules! deps {
    () => {
        BacktraceFrameFmt!();
        BacktraceFmt!();
        BytesOrWideString!();
        PrintFmt!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a , 'b > BacktraceFmt < 'a , 'b > { # [doc = " Create a new `BacktraceFmt` which will write output to the provided"] # [doc = " `fmt`."] # [doc = ""] # [doc = " The `format` argument will control the style in which the backtrace is"] # [doc = " printed, and the `print_path` argument will be used to print the"] # [doc = " `BytesOrWideString` instances of filenames. This type itself doesn't do"] # [doc = " any printing of filenames, but this callback is required to do so."] pub fn new (fmt : & 'a mut fmt :: Formatter < 'b > , format : PrintFmt , print_path : & 'a mut (dyn FnMut (& mut fmt :: Formatter < '_ > , BytesOrWideString < '_ >) -> fmt :: Result + 'b) ,) -> Self { BacktraceFmt { fmt , frame_index : 0 , format , print_path , } } # [doc = " Prints a preamble for the backtrace about to be printed."] # [doc = ""] # [doc = " This is required on some platforms for backtraces to be fully"] # [doc = " symbolicated later, and otherwise this should just be the first method"] # [doc = " you call after creating a `BacktraceFmt`."] pub fn add_context (& mut self) -> fmt :: Result { # [cfg (target_os = "fuchsia")] fuchsia :: print_dso_context (self . fmt) ? ; Ok (()) } # [doc = " Adds a frame to the backtrace output."] # [doc = ""] # [doc = " This commit returns an RAII instance of a `BacktraceFrameFmt` which can be used"] # [doc = " to actually print a frame, and on destruction it will increment the"] # [doc = " frame counter."] pub fn frame (& mut self) -> BacktraceFrameFmt < '_ , 'a , 'b > { BacktraceFrameFmt { fmt : self , symbol_index : 0 , } } # [doc = " Completes the backtrace output."] # [doc = ""] # [doc = " This is currently a no-op but is added for future compatibility with"] # [doc = " backtrace formats."] pub fn finish (& mut self) -> fmt :: Result { # [cfg (target_os = "fuchsia")] fuchsia :: finish_context (self . fmt) ? ; Ok (()) } # [doc = " Inserts a message in the backtrace output."] # [doc = ""] # [doc = " This allows information to be inserted between frames,"] # [doc = " and won't increment the `frame_index` unlike the `frame`"] # [doc = " method."] pub fn message (& mut self , msg : & str) -> fmt :: Result { self . fmt . write_str (msg) } # [doc = " Return the inner formatter."] # [doc = ""] # [doc = " This is used for writing custom information between frames with `write!` and `writeln!`,"] # [doc = " and won't increment the `frame_index` unlike the `frame` method."] pub fn formatter (& mut self) -> & mut fmt :: Formatter < 'b > { self . fmt } }
    };
}

impl_39!()