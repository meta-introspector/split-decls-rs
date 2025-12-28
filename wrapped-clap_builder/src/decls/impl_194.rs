macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl From < & '_ Str > for Str { fn from (id : & '_ Str) -> Self { id . clone () } }
    };
}

impl_194!()