macro_rules! deps {
    () => {
        Guard!();
        Collector!();
        LocalHandle!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl LocalHandle { # [doc = " Pins the handle."] # [inline] pub fn pin (& self) -> Guard { unsafe { (* self . local) . pin () } } # [doc = " Returns `true` if the handle is pinned."] # [inline] pub fn is_pinned (& self) -> bool { unsafe { (* self . local) . is_pinned () } } # [doc = " Returns the `Collector` associated with this handle."] # [inline] pub fn collector (& self) -> & Collector { unsafe { (* self . local) . collector () } } }
    };
}

impl_76!();