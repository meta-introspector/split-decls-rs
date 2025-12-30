// Generated macro for transmute_lt (function)
macro_rules! Depcrate_rt_schedulertransmute_lt {
() => {
// Module: crate::rt::scheduler
// Provides: {"transmute_lt"}
// Dependencies: {}
unsafe fn transmute_lt < 'a , 'b > (state : & 'a RefCell < State < 'b > >) -> & 'a RefCell < State < 'static > > { :: std :: mem :: transmute (state) }
};
}
