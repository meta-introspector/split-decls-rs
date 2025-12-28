macro_rules! deps {
    () => {
        Counters!();
    };
}

macro_rules! instrument {
    () => {
        deps!();
        # [doc = " A simple macro for conditionally executing instrumentation logic when"] # [doc = " the 'trace' log level is enabled. This is a compile-time no-op when the"] # [doc = " 'internal-instrument-pikevm' feature isn't enabled. The intent here is that"] # [doc = " this makes it easier to avoid doing extra work when instrumentation isn't"] # [doc = " enabled."] # [doc = ""] # [doc = " This macro accepts a closure of type `|&mut Counters|`. The closure can"] # [doc = " then increment counters (or whatever) in accordance with what one wants"] # [doc = " to track."] macro_rules ! instrument { ($ fun : expr) => { # [cfg (feature = "internal-instrument-pikevm")] { let fun : & mut dyn FnMut (& mut Counters) = & mut $ fun ; COUNTERS . with (| c : & RefCell < Counters >| fun (& mut * c . borrow_mut ())) ; } } ; }
    };
}

instrument!();