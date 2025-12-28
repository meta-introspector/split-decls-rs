macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < T > FusedFuture for Ready < T > { fn is_terminated (& self) -> bool { self . 0 . is_none () } }
    };
}

impl_194!();