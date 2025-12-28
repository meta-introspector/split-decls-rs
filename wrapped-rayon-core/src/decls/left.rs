macro_rules! left {
    () => {
        # [doc = " ```compile_fail,E0277\n\nuse std::rc::Rc;\n\nrayon_core::join(|| Rc::new(22), || ()); //~ ERROR\n\n``` "] mod left { }
    };
}

left!()