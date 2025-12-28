macro_rules! deps {
    () => {
        AbortHandle!();
        AbortInner!();
    };
}

macro_rules! macro_1333 {
    () => {
        deps!();
        pin_project ! { # [doc = " A future/stream which can be remotely short-circuited using an `AbortHandle`."] # [derive (Debug , Clone)] # [must_use = "futures/streams do nothing unless you poll them"] pub struct Abortable < T > { # [pin] task : T , inner : Arc < AbortInner >, } }
    };
}

macro_1333!();