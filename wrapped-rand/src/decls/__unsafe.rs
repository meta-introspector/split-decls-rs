macro_rules! __unsafe {
    () => {
        # [doc = " Call target for unsafe macros"] const unsafe fn __unsafe () { }
    };
}

__unsafe!()