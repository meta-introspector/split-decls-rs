macro_rules! my_module {
    () => {
        # [doc = " Mock this entire module"] # [automock] pub mod my_module { # [doc = " A function in a mocked module"] pub fn modfunc () { unimplemented ! () } }
    };
}

my_module!()