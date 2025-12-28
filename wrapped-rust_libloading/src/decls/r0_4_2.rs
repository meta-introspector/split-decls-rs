macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! r0_4_2 {
    () => {
        deps!();
        # [doc = " Release 0.4.2 (2017-09-24)"] # [doc = ""] # [doc = " * Improved error and race-condition handling on Windows;"] # [doc = " * Improved documentation about thread-safety of Library;"] # [doc = " * Added `Symbol::<Option<T>::lift_option() -> Option<Symbol<T>>` convenience method."] pub mod r0_4_2 { }
    };
}

r0_4_2!();