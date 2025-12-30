// Generated macro for Notify (type)
macro_rules! Depcrate_buildNotify {
() => {
// Module: crate::build
// Provides: {"Notify"}
// Dependencies: {}
# [doc = " Checkout notifications callback."] # [doc = ""] # [doc = " The first argument is the notification type, the next is the path for the"] # [doc = " the notification, followed by the baseline diff, target diff, and workdir diff."] # [doc = ""] # [doc = " The callback must return a bool specifying whether the checkout should"] # [doc = " continue."] pub type Notify < 'a > = dyn FnMut (CheckoutNotificationType , Option < & Path > , Option < DiffFile < '_ > > , Option < DiffFile < '_ > > , Option < DiffFile < '_ > > ,) -> bool + 'a ;
};
}
