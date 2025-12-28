macro_rules! macro_893 {
    () => {
        bitflags ! { # [doc = " Types of notifications emitted from checkouts."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct CheckoutNotificationType : u32 { # [doc = " Notification about a conflict."] const CONFLICT = raw :: GIT_CHECKOUT_NOTIFY_CONFLICT as u32 ; # [doc = " Notification about a dirty file."] const DIRTY = raw :: GIT_CHECKOUT_NOTIFY_DIRTY as u32 ; # [doc = " Notification about an updated file."] const UPDATED = raw :: GIT_CHECKOUT_NOTIFY_UPDATED as u32 ; # [doc = " Notification about an untracked file."] const UNTRACKED = raw :: GIT_CHECKOUT_NOTIFY_UNTRACKED as u32 ; # [doc = " Notification about an ignored file."] const IGNORED = raw :: GIT_CHECKOUT_NOTIFY_IGNORED as u32 ; } }
    };
}

macro_893!()