macro_rules! right {
    () => {
        # [doc = " ```compile_fail,E0277\n\nuse std::rc::Rc;\n\nrayon_core::join(|| (), || Rc::new(23)); //~ ERROR\n\n``` "] mod right { }
    };
}

right!();