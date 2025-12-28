macro_rules! ToGraph6 {
    () => {
        # [doc = " A graph that can be converted to graph6 format string."] pub trait ToGraph6 { fn graph6_string (& self) -> String ; }
    };
}

ToGraph6!();