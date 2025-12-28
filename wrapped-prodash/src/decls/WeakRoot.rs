macro_rules! deps {
    () => {
        Root!();
    };
}

macro_rules! WeakRoot {
    () => {
        deps!();
        # [doc = " The top-level root as weak handle, which needs an upgrade to become a usable root."] # [doc = ""] # [doc = " If the underlying reference isn't present anymore, such upgrade will fail permanently."] pub trait WeakRoot { # [doc = " The type implementing the `Root` trait"] type Root : Root ; # [doc = " Equivalent to `std::sync::Weak::upgrade()`."] fn upgrade (& self) -> Option < Self :: Root > ; }
    };
}

WeakRoot!()