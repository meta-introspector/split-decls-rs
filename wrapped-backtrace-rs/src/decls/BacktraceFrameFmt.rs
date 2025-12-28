macro_rules! deps {
    () => {
        BacktraceFmt!();
    };
}

macro_rules! BacktraceFrameFmt {
    () => {
        deps!();
        # [doc = " A formatter for just one frame of a backtrace."] # [doc = ""] # [doc = " This type is created by the `BacktraceFmt::frame` function."] pub struct BacktraceFrameFmt < 'fmt , 'a , 'b > { fmt : & 'fmt mut BacktraceFmt < 'a , 'b > , symbol_index : usize , }
    };
}

BacktraceFrameFmt!()